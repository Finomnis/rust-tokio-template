mod command_line;
mod dummy_task;
mod shutdown_trigger;

use anyhow::Result;
use log;
use shutdown_trigger::{start_submodule, shutdown_with_timeout};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Query command line options and initialize logging
    let _opts = command_line::parse();

    // Register Ctrl+C and SIGTERM handlers
    shutdown_trigger::register_signal_handlers();

    // Actual program
    log::info!("Hello, world!");
    let dummy_task_handle = start_submodule(dummy_task::dummy_task());

    // Wait for signals
    shutdown_trigger::wait_for_shutdown().await;

    // Shutdown procedure
//     shutdown_with_timeout(Duration::from_secs(5),
//         dummy_task_handle
// );

    log::info!("Shutting down ...");
    log::info!("Shutdown successful.");
    Ok(())
}
