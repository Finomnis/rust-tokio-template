use super::graceful_shutdown::wait_until_shutdown;
use anyhow::{anyhow, Result};
use log;
use tokio::time::{sleep, Duration};

pub async fn dummy_task() -> Result<()> {
    log::info!("This task will fail in 10 seconds.");

    async fn countdown() {
        for i in (1..=10).rev() {
            log::info!("{}", i);
            sleep(Duration::from_secs(1)).await;
        }
    }

    tokio::select! {
        _ = countdown() => Err(anyhow!("TASK FAILED, as expected.")),
        _ = wait_until_shutdown() => {
            log::info!("Shutting down dummy task!");
            sleep(Duration::from_millis(500)).await;
             Ok(())
        },
    }
}
