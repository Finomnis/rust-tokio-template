use anyhow::Result;
use lazy_static::lazy_static;
use std::future::Future;
use tokio_util::sync::{CancellationToken, DropGuard};

/// Waits for a signal that requests a graceful shutdown, like SIGTERM or SIGINT.
#[cfg(unix)]
fn wait_for_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    // Shut down on context exit
    let _shutdown_guard = create_shutdown_guard();

    let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
    let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();

    tokio::select!(
        e = signal_terminate.recv() => {log::info!("Received SIGTERM."); e},
        e = signal_interrupt.recv() => {log::info!("Received SIGINT."); e},
    );
}

/// Waits for a signal that requests a graceful shutdown, Ctrl-C (SIGINT).
#[cfg(windows)]
async fn wait_for_signal() {
    use tokio::signal::ctrl_c;

    // Shut down on context exit
    let _shutdown_guard = create_shutdown_guard();

    ctrl_c().await.unwrap();
    log::info!("Received SIGINT.");
}

/// Registers Ctrl+Z and SIGTERM handlers to cause a program shutdown
pub fn register_signal_handlers() {
    tokio::spawn(wait_for_signal());
}

// Signals global shutdown
lazy_static! {
    static ref SHUTDOWN_TOKEN: CancellationToken = CancellationToken::new();
}

/// Waits asynchronously until a program shutdown was initiated
pub async fn wait_for_shutdown() {
    SHUTDOWN_TOKEN.cancelled().await;
}

/// Creates a guard object that triggers a program shutdown when dropped
pub fn create_shutdown_guard() -> DropGuard {
    SHUTDOWN_TOKEN.clone().drop_guard()
}

/// Executes an async submodule.
///
/// When the submodule returns an error,
/// a program shutdown gets triggered.
pub fn start_submodule(
    submodule: impl Future<Output = Result<()>> + Send + 'static,
) -> tokio::task::JoinHandle<()> {
    async fn submodule_executor(submodule: impl Future<Output = Result<()>>) {
        if let Err(e) = submodule.await {
            log::error!("Submodule Error: {:?}", e);
            SHUTDOWN_TOKEN.cancel();
        }
    }

    tokio::spawn(submodule_executor(submodule))
}
