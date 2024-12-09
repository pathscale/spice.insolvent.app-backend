use alloy_primitives::bytes;
use ethers::types::U256;
use ethers::{prelude::BlockId, providers::Middleware};
use spice_backend::api::*;
use spice_backend::tables::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::{env, process};
use sysinfo::{Pid, System};
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
use clap::Parser;



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
       #[arg(short, long)]
    start_block: u32,

        #[arg(short, long)]
    end_block: u32,

    #[arg(short, long)]
    url: String,
}

fn u256_to_u64_quads(value: U256) -> [u64; 4] {
    let mut quads = [0u64; 4];

    for i in 0..4 {
        quads[3 - i] = (value >> (i * 64)).low_u64();
    }

    quads
}

async fn check_memory_usage(sys: Arc<Mutex<System>>) {
    let mut sys = sys.lock().await;
    sys.refresh_all();

    let pid = process::id();
    if let Some(process) = sys.process(Pid::from(pid as usize)) {
        let memory_usage = process.memory() / 1_048_576; // Memory usage in megabytes
        println!(
            "Current process (PID: {}): {} MB of memory used",
            pid, memory_usage
        );
    } else {
        println!("Process not found");
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO) //set this to info to debug
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args =  Args::parse();
    println!("args: {args:?}");
    if args.len() != 4 {
        panic!("Accepts two arguments: start_block end_block");
    }

    let start_block = args.start_block;
    let end_block = args.end_block;
    info!("Processing blocks from {} to {}", start_block, end_block);
    let url = "http://127.0.0.1:8545".to_string();
    let api = EthersClient::new(&url, Some("https://eth.llamarpc.com"));

    let block_table = BlockWorkTable::default();
    let tx_table = TransactionWorkTable::default();
    let current_id = AtomicU32::new(0);

    let mut num_transactions: u128 = 0;

    let mut last_time = SystemTime::now();
    let mut txs_at_last_timer = num_transactions;

    // Create a new System object
    let mut sys = Arc::new(Mutex::new(System::new_all()));

    for block_number in start_block..=end_block {
        match api
            .get_block_with_txs(BlockId::from(block_number as u64))
            .await
        {
            Ok(Some(block)) => {
                let block_id = current_id.fetch_add(1, Ordering::SeqCst);

                //info!("Processing block: {:?}", block.number);
                let mut tx_ids: Vec<u32> = vec![];

                for tx in &block.transactions {
                    num_transactions += 1;
                    let tx_id = current_id.fetch_add(1, Ordering::SeqCst);
                    tx_ids.push(tx_id);
                    let value_quads = u256_to_u64_quads(tx.value);
                    let fee_quads = u256_to_u64_quads(
                        U256::from(tx.gas.as_u64()) * tx.gas_price.unwrap_or_default(),
                    );
                    let gas_price_quads = u256_to_u64_quads(tx.gas_price.unwrap_or_default());

                    //info!("Transaction details: {:?}", tx);

                    tx_table.insert(TransactionRow {
                        id: tx_id,
                        hash: tx.hash.into(),
                        status: "processed".to_string(),
                        block_number,
                        timestamp_s: block.timestamp.as_u32(),
                        from_address: tx.from.into(),
                        to_address: tx.to.map_or([0u8; 20], |h160| h160.into()),
                        internal_transactions: "".to_string(), //TODO: this needs to be fetched
                        value_high: value_quads[3],
                        value_mid_high: value_quads[2],
                        value_mid_low: value_quads[1],
                        value_low: value_quads[0],
                        fee_high: fee_quads[3], // ugh
                        fee_mid_high: fee_quads[2],
                        fee_mid_low: fee_quads[1],
                        fee_low: fee_quads[0],
                        gas_price_high: gas_price_quads[3],
                        gas_price_mid_high: gas_price_quads[2],
                        gas_price_mid_low: gas_price_quads[1],
                        gas_price_low: gas_price_quads[0],
                    })?;
                    //info!("Transaction inserted with ID: {}", tx_id);

                    //info!("Transaction table after insertion:\n{:#?}", tx_table);
                    //                 info!("transactions_since_last_timing_event: {}",
                    // transactions_since_last_timing_event);
                    let now = SystemTime::now();
                    if SystemTime::now().duration_since(last_time)?.as_secs() >= 1 {
                        last_time = now;
                        info!("Processing {} tps", num_transactions - txs_at_last_timer);
                        txs_at_last_timer = num_transactions;

                        let sysclone = sys.clone();

                        tokio::spawn(async move {
                            check_memory_usage(sysclone).await;
                        });
                    }

                    if num_transactions % 10_000 == 0 {
                        info!("Processed {num_transactions} transactions");
                    }
                }

                block_table.insert(BlockRow {
                    id: block_id,
                    number: block.number.unwrap_or_default().as_u32(),
                    status: 1u8,
                    timestamp_s: block.timestamp.as_u32(),
                    transactions: serde_json::to_string(&tx_ids)?,
                    eth_price_usd_cents: 0, //TODO: use cmc lookup here
                })?;

                // if block_number % 10_000 == 0 {
                //     info!("Block inserted with NUMBER: {}", block_number);
                // }

                //info!("Block table after insertion:\n{:#?}", block_table);
            }
            Ok(None) => error!("Block {} not found", block_number),
            Err(e) => error!("Error fetching block {}: {}", block_number, e),
        }
    }
    info!("Processed all blocks");
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
