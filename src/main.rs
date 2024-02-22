use anyhow::Result;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    main,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

mod round_robin;
mod utils;

use round_robin::RoundRobin;
use utils::spawn_and_log_error;

const ADDR: &str = "127.0.0.1:8080";

async fn handle_client(mut stream: TcpStream, round_robin: Arc<Mutex<RoundRobin>>) -> Result<()> {
    let mut buf = [0; 1024];
    let mut msg = String::new();

    while let Ok(n) = stream.read(&mut buf).await {
        if n == 0 {
            break;
        }
        msg.push_str(&String::from_utf8_lossy(&buf[..n]));

        if msg.contains("\r\n\r\n") {
            break;
        }
    }
    println!("{msg}");

    let response = round_robin.lock().await.write(&buf).await?;

    stream.write_all(&response).await?;
    stream.shutdown().await?;

    Ok(())
}

async fn run(backends: Vec<String>) -> Result<()> {
    let listener = TcpListener::bind(ADDR).await?;

    let rr = RoundRobin::new(backends);
    let rr = Arc::new(Mutex::new(rr));

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Received request from {}", socket.peer_addr()?);

        tokio::spawn(spawn_and_log_error(handle_client(socket, rr.clone())));
    }
}

#[main]
async fn main() -> Result<()> {
    let backend = vec!["127.0.0.1:8081".to_string(), "127.0.0.1:8082".to_string()];

    run(backend).await?;

    Ok(())
}
