use clap::Parser;
use ethers::{prelude::BlockId, providers::Middleware};
use spice_backend::api::*;
use spice_backend::tables::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use sysinfo::System;
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
use spice_backend::check_memory_usage;


#[cfg(feature = "jemalloc")]
use jemallocator::Jemalloc;
#[cfg(feature = "mimallocator")]
use mimalloc::MiMalloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
#[cfg(feature = "mimallocator")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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



#[tokio::main]
async fn main() -> eyre::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO) //set this to info to debug
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args = Args::parse();
    println!("args: {args:?}");

    let start_block = args.start_block;
    let end_block = args.end_block;
    info!("Processing blocks from {} to {}", start_block, end_block);
    let url = args.url;
    let api = EthersClient::new(&url, Some("https://eth.llamarpc.com"));

    //let block_table = BlockWorkTable::default();
    let tx_table = TransactionWorkTable::default();
    let current_id = AtomicU32::new(0);

    let mut num_transactions: u128 = 0;

    let mut last_time = SystemTime::now();
    let mut txs_at_last_timer = num_transactions;

    // Create a new System object
    let sys = Arc::new(Mutex::new(System::new_all()));

    for block_number in start_block..=end_block {
        match api
            .get_block_with_txs(BlockId::from(block_number as u64))
            .await
        {
            Ok(Some(block)) => {
                //let block_id = current_id.fetch_add(1, Ordering::SeqCst);

                //info!("Processing block: {:?}", block.number);
                let mut tx_ids: Vec<u32> = vec![];

                for tx in &block.transactions {
                    num_transactions += 1;
                    let tx_id = current_id.fetch_add(1, Ordering::SeqCst);
                    tx_ids.push(tx_id);

                    //info!("Transaction details: {:?}", tx);

                    tx_table.insert(TransactionRow {
                        id: tx_id,
                        hash: tx.hash.into(),
                        status: 1u8,
                        block_number,
                        timestamp_s: block.timestamp.as_u32(),
                        from_address: tx.from.into(),
                        to_address: tx.to.map(|address| address.into()),
                        //internal_transactions: "".to_string(), //TODO: this needs to be fetched
                        value: tx.value.into(),
                        fee: tx.gas.saturating_mul(tx.gas_price.unwrap_or_default()).into(),
                        gas: tx.gas_price.map(|gas_price| gas_price.into()),
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

                    if num_transactions % 50_000 == 0 {
                        info!("Processed {num_transactions} transactions");
                    }
                }

                // block_table.insert(BlockRow {
                //     id: block_id,
                //     number: block.number.unwrap_or_default().as_u32(),
                //     status: 1u8,
                //     timestamp_s: block.timestamp.as_u32(),
                //     transactions: tx_ids,
                //     eth_price_usd_cents: 0, //TODO: use cmc lookup here
                // })?;

                // if block_number % 10_000 == 0 {
                //      debug!("Block inserted with NUMBER: {}", block_number);
                //  }

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
