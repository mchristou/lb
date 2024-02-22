use anyhow::Result;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Debug)]
pub(crate) struct RoundRobin {
    backends: Vec<String>,
    current_index: usize,
}

impl RoundRobin {
    pub(crate) fn new(backends: Vec<String>) -> Self {
        RoundRobin {
            backends,
            current_index: 0,
        }
    }

    pub(crate) async fn write(&mut self, bytes: &[u8]) -> Result<Vec<u8>> {
        if let Some(backend) = self.select_backend() {
            let mut backend_stream = TcpStream::connect(backend.to_string()).await?;
            backend_stream.write_all(bytes).await?; // Write response data

            let mut response = Vec::new();
            backend_stream.read_to_end(&mut response).await?; // Read response from backend

            backend_stream.shutdown().await?;
            Ok(response)
        } else {
            Err(anyhow::anyhow!("There is no available server"))
        }
    }

    fn select_backend(&mut self) -> Option<String> {
        if self.backends.is_empty() {
            return None;
        }

        let backend = self.backends[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.backends.len();

        Some(backend)
    }
}
