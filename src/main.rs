use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const ADDR: &str = "127.0.0.1:8080";

async fn run() -> Result<()> {
    let listener = TcpListener::bind(ADDR).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Received request from {}", socket.peer_addr()?);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut msg = String::new();

            // Read data from the socket
            while let Ok(n) = socket.read(&mut buf).await {
                if n == 0 {
                    break;
                }
                msg.push_str(&String::from_utf8_lossy(&buf[..n]));

                if msg.contains("\r\n\r\n") {
                    break;
                }
            }

            // Log the received message
            println!("{}", msg);

            // Echo the message back to the client
            if let Err(e) = socket.write_all(msg.as_bytes()).await {
                eprintln!("Error writing to socket: {}", e);
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}
