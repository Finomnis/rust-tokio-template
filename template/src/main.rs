mod command_line;
mod dummy_task;

use anyhow::Result;
use log;
use tokio::time::Duration;
use tokio_graceful_shutdown::{
    register_signal_handlers, start_submodule, wait_for_submodule_shutdown,
    wait_until_shutdown_started,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Query command line options and initialize logging
    let _opts = command_line::parse();

    // Register Ctrl+C and SIGTERM handlers
    register_signal_handlers();

    // Actual program
    log::info!("Hello, world!");
    let dummy_task_handle = start_submodule(dummy_task::dummy_task());

    // Wait for program shutdown initiation
    wait_until_shutdown_started().await;

    // Wait until all submodules have shut down
    wait_for_submodule_shutdown!(Duration::from_millis(1000), dummy_task_handle)
}
