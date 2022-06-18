mod command_line;
mod dummy_task;

use miette::Result;
use tokio::time::Duration;
use tokio_graceful_shutdown::Toplevel;

#[tokio::main]
async fn main() -> Result<()> {
    // Query command line options and initialize logging
    let _opts = command_line::parse();

    // Initialize and run subsystems
    Toplevel::new()
        .start("dummy_task", dummy_task::dummy_task)
        .catch_signals()
        .handle_shutdown_requests(Duration::from_millis(1000))
        .await
        .map_err(Into::into)
}
