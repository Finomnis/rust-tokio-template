use miette::Result;
use tokio_graceful_shutdown::SubsystemHandle;

pub async fn dummy_task(subsys: SubsystemHandle) -> Result<()> {
    log::info!("dummy_task started.");
    subsys.on_shutdown_requested().await;
    log::info!("dummy_task stopped");

    Ok(())
}
