use std::{process, sync::Arc};

use sysinfo::{Pid, System};
use tokio::sync::Mutex;

pub mod tables;
pub mod api;
pub mod rkyv_wrappers;

pub async fn check_memory_usage(sys: Arc<Mutex<System>>) {
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