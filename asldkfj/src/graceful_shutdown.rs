use anyhow::Result;
use lazy_static::lazy_static;
use std::future::Future;
use tokio_util::sync::CancellationToken;

/// Waits for a signal that requests a graceful shutdown, like SIGTERM or SIGINT.
#[cfg(unix)]
fn wait_for_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
    let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();

    tokio::select!(
        e = signal_terminate.recv() => {log::info!("Received SIGTERM."); e},
        e = signal_interrupt.recv() => {log::info!("Received SIGINT."); e},
    );

    initiate_shutdown();
}

/// Waits for a signal that requests a graceful shutdown, Ctrl-C (SIGINT).
#[cfg(windows)]
async fn wait_for_signal() {
    use tokio::signal::ctrl_c;

    ctrl_c().await.unwrap();
    log::info!("Received SIGINT.");

    initiate_shutdown();
}

/// Registers Ctrl+C and SIGTERM handlers to cause a program shutdown
pub fn register_signal_handlers() {
    tokio::spawn(wait_for_signal());
}

// Signals global shutdown
lazy_static! {
    static ref SHUTDOWN_TOKEN: CancellationToken = CancellationToken::new();
}

/// Waits asynchronously until a program shutdown was initiated
pub async fn wait_until_shutdown() {
    SHUTDOWN_TOKEN.cancelled().await;
}

/// Initiates a shutdown
pub fn initiate_shutdown() {
    log::info!("Initiating shutdown ...");
    SHUTDOWN_TOKEN.cancel();
}

/// Executes an async submodule.
///
/// When the submodule returns an error,
/// a program shutdown gets triggered.
pub fn start_submodule(
    submodule: impl Future<Output = Result<()>> + Send + 'static,
) -> tokio::task::JoinHandle<Result<()>> {
    async fn submodule_executor(submodule: impl Future<Output = Result<()>>) -> Result<()> {
        let result = submodule.await;
        if let Err(e) = &result {
            log::error!("Submodule Error: {}", e);
            initiate_shutdown();
        }
        result
    }

    tokio::spawn(submodule_executor(submodule))
}


#[macro_export]
/// Waits for given task handles. Times out after given duration.
macro_rules! shutdown_with_timeout {
    ($duration:expr, $($handles : expr),* $(,) ?) => {{
        use anyhow::anyhow;

        // Flattens JoinHandle<T> to Future<Result<T>>, to enable proper error early stopping in try_join.
        async fn flatten(handle: tokio::task::JoinHandle<anyhow::Result<()>>) -> anyhow::Result<()> {
            match handle.await {
                Ok(Ok(result)) => Ok(result),
                Ok(Err(err)) => Err(err),
                Err(err) => Err(anyhow::Error::new(err)),
            }
        }

        let task_joiner = async {
            tokio::try_join!(
                $(flatten($handles)),*
            ).and(Ok(()))
        };

        let result = tokio::select! {
            e = task_joiner => e,
            _ = tokio::time::sleep($duration) => Err(anyhow::anyhow!("Subsystem shutdown took too long!"))
        };

        match result {
            Err(e) => {
                log::error!("Submodule Error: {:?}", e);
                Err(anyhow!("Submodule failure."))
            }
            Ok(()) => {
                log::info!("Subsystems shut down successfully.");
                Ok(())
            }
        }
    }};
}
