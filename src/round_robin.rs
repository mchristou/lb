use anyhow::Result;
use std::sync::Arc;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) struct RoundRobin {
    backends: Vec<Arc<Mutex<TcpStream>>>,
    current_index: usize,
}

impl RoundRobin {
    pub(crate) fn new(backends: Vec<TcpStream>) -> Self {
        let backends = backends
            .into_iter()
            .map(|b| Arc::new(Mutex::new(b)))
            .collect();

        RoundRobin {
            backends,
            current_index: 0,
        }
    }

    pub(crate) async fn write(&mut self, bytes: &[u8]) -> Result<Vec<u8>> {
        if let Some(backend) = self.select_backend() {
            let mut backend = backend.lock().await;

            backend.write_all(bytes).await?;
            backend.flush().await?;

            let mut response = Vec::new();
            backend.read_to_end(&mut response).await?;

            Ok(response)
        } else {
            Err(anyhow::anyhow!("There is no available server"))
        }
    }

    fn select_backend(&mut self) -> Option<Arc<Mutex<TcpStream>>> {
        if self.backends.is_empty() {
            return None;
        }

        let backend = self.backends[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.backends.len();

        Some(backend)
    }
}
