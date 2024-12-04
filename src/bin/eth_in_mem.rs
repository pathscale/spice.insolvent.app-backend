use ethers::{prelude::BlockId, providers::Middleware};
use spice_backend::api::*;
use spice_backend::tables::*;
use std::error::Error;
use std::{env, fs::File, io::BufWriter};

use std::sync::atomic::{AtomicU64, Ordering};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Accepts two arguments: start_block end_block");
    }

    let start_block = args[1].parse::<u64>()?;
    let end_block = args[2].parse::<u64>()?;
    println!("Processing blocks from {} to {}", start_block, end_block);
    let url = "https://mainnet.infura.io/v3/28a2f37d92a942c5951f4a00b431432c".to_string();
    let api = EthersClient::new(&url, Some("https://eth.llamarpc.com"));

    let mut block_table = BlockWorkTable::default();
    let mut tx_table = TransactionWorkTable::default();
    let current_id = AtomicU64::new(0);

    for block_number in start_block..=end_block {
        match api.get_block_with_txs(BlockId::from(block_number)).await {
            Ok(Some(block)) => {
                let block_id = current_id.fetch_add(1, Ordering::SeqCst);

                println!("Processing block: {:?}", block.number);
                let block_json = serde_json::to_string(&block)?;
                println!("Block JSON: {}", block_json);

                block_table.insert(BlockRow {
                    id: block_id,
                    number: block.number.unwrap_or_default().as_u64(),
                    status: "fetched".to_string(),
                    timestamp_s: block.timestamp.as_u64(),
                    transactions: block_json,
                    eth_price_usd_cents: 0, //TODO:  add cmc here
                })?;
                println!("Block inserted with NUMBER: {}", block_number);

                println!("Block table after insertion: {:?}", block_table);

                for tx in &block.transactions {
                    let tx_id = current_id.fetch_add(1, Ordering::SeqCst);

                    println!("Transaction details: {:?}", tx);

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
                        internal_transactions: "".to_string(), //need to fetch these
                        value: tx.value.as_u64(),
                        fee: tx.gas.as_u64() * tx.gas_price.unwrap_or_default().as_u64(),
                        gas_price: tx.gas_price.unwrap_or_default().as_u64(),
                    })?;
                    println!("Transaction inserted with ID: {}", tx_id);

                    println!("Transaction table after insertion: {:?}", tx_table);
                }
            }
            Ok(None) => eprintln!("Block {} not found", block_number),
            Err(e) => eprintln!("Error fetching block {}: {}", block_number, e),
        }
    }
    Ok(())
}
