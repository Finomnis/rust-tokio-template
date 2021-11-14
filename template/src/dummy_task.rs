use anyhow::Result;
use log;
use tokio_graceful_shutdown::wait_until_shutdown_started;

pub async fn dummy_task() -> Result<()> {
    log::info!("dummy_task started.");

    wait_until_shutdown_started().await;

    log::info!("dummy_task stopped");

    Ok(())
}
