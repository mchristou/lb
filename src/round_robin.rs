use std::sync::Arc;

use anyhow::Result;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
    time,
};

#[derive(Debug)]
enum State {
    Available,
    Unavailable,
}

#[derive(Debug)]
struct Server {
    url: String,
    state: State,
}

#[derive(Debug)]
pub(crate) struct RoundRobin {
    backends: Vec<Arc<Mutex<Server>>>,
    current_index: usize,
}

impl RoundRobin {
    pub(crate) fn new(backends: Vec<String>) -> Self {
        let backends: Vec<_> = backends
            .into_iter()
            .map(|b| {
                Arc::new(Mutex::new(Server {
                    url: b,
                    state: State::Unavailable,
                }))
            })
            .collect();

        for server in backends.iter() {
            let server = Arc::clone(server);

            tokio::spawn(health_check(server));
        }

        RoundRobin {
            backends,
            current_index: 0,
        }
    }

    pub(crate) async fn write(&mut self, bytes: &[u8]) -> Result<Vec<u8>> {
        if let Some(backend) = self.select_backend().await {
            let mut backend_stream = TcpStream::connect(backend.to_string()).await?;
            backend_stream.write_all(bytes).await?;

            let mut response = Vec::new();
            backend_stream.read_to_end(&mut response).await?;

            backend_stream.shutdown().await?;

            Ok(response)
        } else {
            Err(anyhow::anyhow!("There is no available server"))
        }
    }

    async fn select_backend(&mut self) -> Option<String> {
        if self.backends.is_empty() {
            return None;
        }

        let mut index = self.current_index;
        for _ in 0..self.backends.len() {
            let backend = self.backends[index].clone();
            let server = backend.lock().await;

            if let State::Available = server.state {
                self.current_index = (index + 1) % self.backends.len();

                return Some(server.url.clone());
            }

            index = (index + 1) % self.backends.len();
        }

        None
    }
}

async fn health_check(server: Arc<Mutex<Server>>) {
    loop {
        let url = format!("http://{}", server.lock().await.url);
        let response = reqwest::get(url).await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    server.lock().await.state = State::Available;
                    println!("Server {} is healthy", server.lock().await.url);
                } else {
                    server.lock().await.state = State::Unavailable;
                    println!("Server {} is unhealthy", server.lock().await.url);
                }
            }
            Err(_) => {
                server.lock().await.state = State::Unavailable;
                println!(
                    "Error occurred while checking server {}",
                    server.lock().await.url
                );
            }
        }

        time::sleep(time::Duration::from_secs(10)).await;
    }
}
