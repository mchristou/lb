use anyhow::Result;
use std::future::Future;

pub(crate) fn spawn_and_log_error<F>(fut: F) -> tokio::task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    tokio::task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{e}");
        }
    })
}
