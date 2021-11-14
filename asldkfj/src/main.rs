mod command_line;
mod dummy_task;
mod graceful_shutdown;

use anyhow::Result;
use graceful_shutdown::{start_submodule, wait_until_shutdown};
use log;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Query command line options and initialize logging
    let _opts = command_line::parse();

    // Register Ctrl+C and SIGTERM handlers
    graceful_shutdown::register_signal_handlers();

    // Actual program
    log::info!("Hello, world!");
    let dummy_task_handle = start_submodule(dummy_task::dummy_task());

    // Wait for program shutdown initiation
    wait_until_shutdown().await;

    // Wait until all submodules have shut down
    shutdown_with_timeout!(
        Duration::from_millis(1000),
        dummy_task_handle
    )
}
