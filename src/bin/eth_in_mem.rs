use ethers::{prelude::BlockId, providers::Middleware};

use ethers::types::U256;
use spice_backend::api::*;
use spice_backend::tables::*;
use std::error::Error;
use std::sync::atomic::{AtomicU64, Ordering};
use std::env;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

fn u256_to_u64_quads(value: U256) -> [u64; 4] {
    let mut quads = [0u64; 4];

    for i in 0..4 {
        quads[3 - i] = (value >> (i * 64)).low_u64();
    }

    quads
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::ERROR) //set this to info to debug
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Accepts two arguments: start_block end_block");
    }

    let start_block = args[1].parse::<u64>()?;
    let end_block = args[2].parse::<u64>()?;
    info!("Processing blocks from {} to {}", start_block, end_block);
    let url = "https://mainnet.infura.io/v3/28a2f37d92a942c5951f4a00b431432c".to_string(); //This
    //should use our node but not suitable for local testing
    let api = EthersClient::new(&url, Some("https://eth.llamarpc.com"));

    let  block_table = BlockWorkTable::default();
    let   tx_table = TransactionWorkTable::default();
    let current_id = AtomicU64::new(0);

    for block_number in start_block..=end_block {
        match api.get_block_with_txs(BlockId::from(block_number)).await {
            Ok(Some(block)) => {
                let block_id = current_id.fetch_add(1, Ordering::SeqCst);

                info!("Processing block: {:?}", block.number);
                let block_json = serde_json::to_string(&block)?;
                info!("Block JSON: {}", block_json);

                block_table.insert(BlockRow {
                    id: block_id,
                    number: block.number.unwrap_or_default().as_u64(),
                    status: "fetched".to_string(),
                    timestamp_s: block.timestamp.as_u64(),
                    transactions: block_json, //TODO: Storing block json as string is potentially
                    //redundant and costly to store
                    eth_price_usd_cents: 0, //TODO: use cmc lookup here
                })?;
                info!("Block inserted with NUMBER: {}", block_number);


                info!("Block table after insertion:\n{:#?}", block_table);

                for tx in &block.transactions {
                    let tx_id = current_id.fetch_add(1, Ordering::SeqCst);
                     let value_quads = u256_to_u64_quads(tx.value);
                    let fee_quads = u256_to_u64_quads(
                        U256::from(tx.gas.as_u64()) * tx.gas_price.unwrap_or_default(),
                    );
                    let gas_price_quads = u256_to_u64_quads(tx.gas_price.unwrap_or_default());

                    info!("Transaction details: {:?}", tx);

                    tx_table.insert(TransactionRow {
                        id: tx_id,
                        hash: tx.hash.to_string(),
                        status: "processed".to_string(),
                        block_number: block_id,
                        timestamp_s: block.timestamp.as_u64(),
                        from_address: tx.from.to_string(),
                        to_address: tx
                            .to
                            .map_or_else(|| "".to_string(), |addr| addr.to_string()),
                        internal_transactions: "".to_string(),//TODO: this needs to be fetched
                        value_high: value_quads[3],
                        value_mid_high: value_quads[2],
                        value_mid_low: value_quads[1],
                        value_low: value_quads[0],
                        fee_high: fee_quads[3],               // ugh
                        fee_mid_high: fee_quads[2],
                        fee_mid_low: fee_quads[1],
                        fee_low: fee_quads[0],
                        gas_price_high: gas_price_quads[3],
                        gas_price_mid_high: gas_price_quads[2],
                        gas_price_mid_low: gas_price_quads[1],
                        gas_price_low: gas_price_quads[0],
                    })?;
                    info!("Transaction inserted with ID: {}", tx_id);


                    info!("Transaction table after insertion:\n{:#?}", tx_table);
                }
            }
            Ok(None) => error!("Block {} not found", block_number),
            Err(e) => error!("Error fetching block {}: {}", block_number, e),
        }
    }
    Ok(())
}
