mod command_line;
mod dummy_task;

use anyhow::Result;
use tokio::time::Duration;
use tokio_graceful_shutdown::Toplevel;

#[tokio::main]
async fn main() -> Result<()> {
    // Query command line options and initialize logging
    let _opts = command_line::parse();

    // Initialize and run subsystems
    Toplevel::new()
        .start("dummy_task", dummy_task::DummyTask {})
        .catch_signals()
        .wait_for_shutdown(Duration::from_millis(1000))
        .await
}
