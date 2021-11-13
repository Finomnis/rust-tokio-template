mod command_line;
mod shutdown_trigger;

use anyhow::Result;
use log;

#[tokio::main]
async fn main() -> Result<()> {
    // Query command line options and initialize logging
    let _opts = command_line::parse();

    // Register Ctrl+C and SIGTERM handlers
    shutdown_trigger::register_signal_handlers().await;

    // Actual program
    log::info!("Hello, world!");

    // Wait for signals
    shutdown_trigger::wait_for_shutdown().await;

    // Shutdown procedure
    log::info!("This is a shutdown task.");

    Ok(())
}
