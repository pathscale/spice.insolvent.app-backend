use ethers::{prelude::BlockId, providers::Middleware};

use spice_backend::api::*;
use std::mem::size_of_val;
use spice_backend::tables::*;
use std::error::Error;
use std::{env, fs::File, io::BufWriter};

use ethers::types::U256;
use std::sync::atomic::{AtomicU64, Ordering};

fn u256_to_u64_quads(value: U256) -> [u64; 4] {
    let mut quads = [0u64; 4];

    for i in 0..4 {
        quads[3 - i] = (value >> (i * 64)).low_u64();
    }

    quads
}
pub fn size_blocks_and_transactions(block_table: BlockWorkTable, tx_table: TransactionWorkTable) -> (usize, usize) {
    let block_size = block_table.get().map(|block| size_of_val(block)).sum();
    let tx_size = tx_table.list().map(|tx| size_of_val(tx)).sum();

    (block_size, tx_size)
}

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
                    eth_price_usd_cents: 0,
                    //TODO:  add cmc here
                })?;
                println!("Block inserted with NUMBER: {}", block_number);

                println!("Block table after insertion: {:?}", block_table);

                for tx in &block.transactions {
                    let tx_id = current_id.fetch_add(1, Ordering::SeqCst);
                    let value_quads = u256_to_u64_quads(tx.value);
                    let fee_quads = u256_to_u64_quads(
                        U256::from(tx.gas.as_u64()) * tx.gas_price.unwrap_or_default(),
                    );
                    let gas_price_quads = u256_to_u64_quads(tx.gas_price.unwrap_or_default());

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
                        value_high: value_quads[3],
                        value_mid_high: value_quads[2],
                        value_mid_low: value_quads[1],
                        value_low: value_quads[0],
                        fee_high: fee_quads[3],
                        fee_mid_high: fee_quads[2],
                        fee_mid_low: fee_quads[1],
                        fee_low: fee_quads[0],
                        gas_price_high: gas_price_quads[3],
                        gas_price_mid_high: gas_price_quads[2],
                        gas_price_mid_low: gas_price_quads[1],
                        gas_price_low: gas_price_quads[0],
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
