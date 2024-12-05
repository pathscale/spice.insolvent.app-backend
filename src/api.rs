use ethers::prelude::*;
use eyre::Result;
use eyre::{Context, ContextCompat};
use serde::Serialize;
use std::ops::Deref;

mod cmc;
mod assets;
mod models;


pub type BlockWithTx = Block<Transaction>;
#[derive(Debug, serde::Deserialize, Serialize, Clone)]
pub enum BlockType {
    Confirmed(u64),
    Mempool,
}

impl ToString for BlockType {
    fn to_string(&self) -> String {
        match self {
            BlockType::Confirmed(block_num) => block_num.to_string(),
            BlockType::Mempool => String::from("Mempool"),
        }
    }
}
pub struct EthersClient {
    client: Provider<Http>,
    fallback_client: Option<Provider<Http>>,
}




impl EthersClient {
    pub fn new(url: &str, fallback_url: Option<&str>) -> Self {
        let client = Provider::<Http>::try_from(url).unwrap();

        let fallback_client = fallback_url.map(|url| Provider::<Http>::try_from(url).unwrap());
        Self {
            client,
            fallback_client,
        }
    }
    pub fn new_debug() -> Self {
        let public_node = "https://ethereum.publicnode.com";
        Self::new(public_node, None)
    }
    pub fn get_main_client(&self) -> &Provider<Http> {
        &self.client
    }
    fn get_providers(&self) -> [Option<&Provider<Http>>; 2] {
        [Some(&self.client), self.fallback_client.as_ref()]
    }

    /// Get upper layer of inner transactions made by a contract
    ///
    /// returns None if data about calls wasn't present at all
    pub async fn get_called_contracts(&self, tx_hash: TxHash) -> Result<Option<Vec<Address>>> {
        let mut error = None;
        for provider in self.get_providers().into_iter().flatten() {
            let result = provider
                .debug_trace_transaction(
                    tx_hash,
                    GethDebugTracingOptions {
                        tracer: Some(GethDebugTracerType::BuiltInTracer(
                            GethDebugBuiltInTracerType::CallTracer,
                        )),
                        ..Default::default()
                    },
                )
                .await;
            match result {
                Ok(trace) => {
                    return if let GethTrace::Known(GethTraceFrame::CallTracer(tr)) = trace {
                        if let Some(calls) = tr.calls {
                            let result = calls
                                .into_iter()
                                .filter_map(|call| call.to.map(|add| add.as_address().cloned()))
                                .filter_map(|add| add)
                                .collect();
                            Ok(Some(result))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                }
                Err(last_err) => {
                    error = Some(last_err);
                    continue;
                }
            }
        }
        Err(error.unwrap())
            .with_context(|| format!("Failed to get called contracts for tx_hash: {:?}", tx_hash))
    }
    pub async fn get_transaction_receipt(&self, hash: TxHash) -> Result<TransactionReceipt> {
        let mut error = None;
        for provider in self.get_providers().into_iter().flatten() {
            let tx = provider.get_transaction_receipt(hash).await?;
            match tx {
                Some(tx) => {
                    return Ok(tx);
                }
                None => {
                    error = Some(eyre::eyre!("Transaction receipt not found for {:?}", hash));
                }
            }
        }
        Err(error.unwrap())
    }
    pub async fn get_block_by_number(&self, block_number: BlockId) -> Result<Option<BlockWithTx>> {
        let block = self.client.get_block_with_txs(block_number).await?;
        Ok(block)
    }
    pub async fn get_latest_block(&self) -> Result<BlockWithTx> {
        let block = self
            .client
            .get_block_with_txs(BlockNumber::Latest)
            .await?
            .with_context(|| format!("Failed to get latest block"))?;
        Ok(block)
    }
    pub async fn get_contract_bytecode(
        &self,
        address: Address,
        block_number: BlockId,
    ) -> Result<ethers::types::Bytes> {
        let byte_code = self.client.get_code(address, Some(block_number)).await?;
        Ok(byte_code)
    }
    pub async fn get_account_balance(
        &self,
        address: Address,
        block_number: BlockId,
    ) -> Result<U256> {
        let balance = self.client.get_balance(address, Some(block_number)).await?;
        Ok(balance)
    }
    pub async fn inspect_txpool(&self) -> Result<TxpoolInspect> {
        let pool = self.client.txpool_inspect().await?;
        Ok(pool)
    }
    pub async fn get_txpool_content(&self) -> Result<TxpoolContent> {
        let content = self.client.txpool_content().await?;
        Ok(content)
    }

    pub async fn get_internal_largest_transfer(
        &self,
        tx_hash: TxHash,
    ) -> Result<Option<(U256, Address)>> {
        let res = self.client.trace_transaction(tx_hash).await?;

        let largest_call = res
            .iter()
            .filter(|trace| match trace.action {
                Action::Call(_) => return true,
                _ => return false,
            })
            // .map(|trace| trace.action)
            .filter_map(|trace| match &trace.action {
                Action::Call(call) => Some(call),
                _ => None,
            })
            .max_by(|a, b| a.value.cmp(&b.value));

        if let Some(call) = largest_call {
            return Ok(Some((call.value, call.to)));
        } else {
            return Ok(None);
        }

        // match res {
        //     GethTrace::Known(geth_trace_frame) => match geth_trace_frame {
        //         GethTraceFrame::Default(default_frame) => {
        //             info!("Got default frame");
        //             let largest_transfer_value = default_frame.struct_logs.iter()
        //             .filter(|log| log.op.starts_with("CALL"))
        //             .filter(|log| log.stack.is_some() && log.stack.as_ref().unwrap().len()> 2)
        //             .reduce(|a, b| if a.stack.as_ref().unwrap()[2] < b.stack.as_ref().unwrap()[2] {
        //                 return b;
        //             } else {
        //                 return a;
        //             });

        //             info!("Largest transfer structlog: {largest_transfer_value:?}");

        //             if largest_transfer_value.is_some() {
        //                 return Ok(Some(largest_transfer_value.unwrap().stack.clone().unwrap()[2]));
        //             } else {
        //                 return Ok(None);
        //             }
        //         },
        //         GethTraceFrame::CallTracer(call_frame) => {
        //             info!("Got call frames: {call_frame:?}");
        //             if let Some(calls) = call_frame.calls {
        //                 let largest_transfer_value = calls.iter()
        //                 .filter(|call| call.typ == "Transfer")
        //                 .reduce(|a,b| {
        //                     if a.value < b.value {
        //                         return b;
        //                     } else {
        //                         return a;
        //                     }
        //                 });

        //                 if largest_transfer_value.is_some() {
        //                     return Ok(largest_transfer_value.unwrap().value);
        //                 } else {
        //                     return Ok(None);
        //                 }
        //             } else {
        //                 return Ok(None);
        //             }
        //         },
        //         _ => {
        //             info!("Got some other kind of frame");
        //             Ok(None)
        //         },
        //     },
        //     GethTrace::Unknown(value) => Ok(None),
        // }
    }

    pub async fn is_contract(&self, address: Address, block_number: &BlockType) -> Result<bool> {
        // Option<BlockId>

        match block_number {
            BlockType::Confirmed(number) => {
                let code = self
                    .client
                    .get_code(
                        address,
                        Some(BlockId::Number(ethers::types::BlockNumber::Number(
                            U64::from(number.clone()),
                        ))),
                    )
                    .await?;
                if !code.is_empty() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            BlockType::Mempool => {
                let code = self.client.get_code(address, None).await?;
                if !code.is_empty() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}

impl Deref for EthersClient {
    type Target = Provider<Http>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
