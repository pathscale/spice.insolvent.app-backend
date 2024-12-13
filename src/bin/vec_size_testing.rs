use std::sync::Arc;
use std::time::SystemTime;

use spice_backend::check_memory_usage;
use sysinfo::System;
use tokio::sync::Mutex;
use tracing::info;


struct DummyRow {
    id: u32,
    number: u64
}

#[tokio::main]

async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let end_number: u32 = 2_200_000_000;

    let mut last_time = SystemTime::now();
    let mut number_at_last_timer = 0;

    let mut vec_table: Vec<DummyRow> = vec![];

    let sys = Arc::new(Mutex::new(System::new_all()));


    for i in 0..=end_number {
        let dummy_row = DummyRow {
            id: 1,
            number: i as u64,
        };

        vec_table.push(dummy_row);

        let now = SystemTime::now();
        if SystemTime::now().duration_since(last_time)?.as_secs() >= 1 {
            last_time = now;
            info!("Processing {} rows/sec", i - number_at_last_timer);
            number_at_last_timer = i;
            
            let sysclone = sys.clone();
            
            info!("Processed {} rows", i+1);
            tokio::spawn(async move {
                check_memory_usage(sysclone).await;
            });
        }
    }

    println!("Finished adding rows");

    Ok(())
}