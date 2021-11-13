use anyhow::Result;
use log;

async fn dummy_task() -> Result<()> {

    log::info("This task will fail in 10 seconds.");

    for i in 0..10{
        log::info("{}", i);
    }

    Err("TASK FAILED, as expected.");
}
