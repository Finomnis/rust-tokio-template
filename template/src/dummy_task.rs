use anyhow::Result;
use async_trait::async_trait;
use tokio_graceful_shutdown::{AsyncSubsystem, SubsystemHandle};

pub struct DummyTask {}

#[async_trait]
impl AsyncSubsystem for DummyTask {
    async fn run(&mut self, subsys: SubsystemHandle) -> Result<()> {
        log::info!("dummy_task started.");
        subsys.on_shutdown_requested().await;
        log::info!("dummy_task stopped");

        Ok(())
    }
}
