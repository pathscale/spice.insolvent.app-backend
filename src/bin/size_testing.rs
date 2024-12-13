use std::sync::Arc;
use std::time::SystemTime;

use spice_backend::check_memory_usage;
use sysinfo::System;
use tokio::sync::Mutex;
use tracing::info;
use worktable::prelude::*;
use worktable::worktable;

worktable!(
    name: SizeTest,
    columns: {
        id: u32 primary_key,
        number: u64,
    }
    indexes: {
        number_idx: number
    }
);

#[tokio::main]

async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let size_test_table = SizeTestWorkTable::default();

    let end_number: u32 = 2_200_000_000;

    let mut last_time = SystemTime::now();
    let mut number_at_last_timer = 0;

    let sys = Arc::new(Mutex::new(System::new_all()));


    for i in 0..=end_number {
        let test_row = SizeTestRow {
            id: i,
            number: i as u64,
        };

        size_test_table.insert(test_row)?;

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
