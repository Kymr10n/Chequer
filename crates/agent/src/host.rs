use anyhow::{Context, Result};
use chequer_common::{Message, TestResults};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Host agent that accepts connections from clients and runs diagnostics
pub struct Host {
    listen_addr: String,
    results: Arc<Mutex<Vec<TestResults>>>,
}

impl Host {
    /// Create a new host instance
    pub fn new(listen_addr: String) -> Self {
        Self {
            listen_addr,
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Start the host server and listen for client connections
    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.listen_addr)
            .await
            .context("Failed to bind to address")?;
        
        info!("Host listening on {}", self.listen_addr);
        info!("Waiting for client connections...");

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    socket.set_nodelay(true).ok();
                    info!("Client connected from {}", addr);
                    let results = Arc::clone(&self.results);
                    
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(socket, results).await {
                            error!("Error handling client {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    warn!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Get collected test results
    pub async fn get_results(&self) -> Vec<TestResults> {
        self.results.lock().await.clone()
    }
}

async fn handle_client(
    mut socket: tokio::net::TcpStream,
    results: Arc<Mutex<Vec<TestResults>>>,
) -> Result<()> {
    let mut buffer = vec![0u8; 8192];

    loop {
        // Read message length (4 bytes)
        let n = socket.read(&mut buffer[..4]).await?;
        if n == 0 {
            info!("Client disconnected");
            break;
        }

        let msg_len = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
        
        if msg_len > buffer.len() {
            buffer.resize(msg_len, 0);
        }

        // Read the actual message
        socket.read_exact(&mut buffer[..msg_len]).await?;
        
        let message: Message = serde_json::from_slice(&buffer[..msg_len])
            .context("Failed to deserialize message")?;

        match message {
            Message::Ping { timestamp } => {
                // Echo back as Pong
                let response = Message::Pong { timestamp };
                send_message(&mut socket, &response).await?;
            }
            Message::TestResults { results: test_results } => {
                info!("Received test results from client");
                results.lock().await.push(test_results);
            }
            Message::Pong { .. } => {
                warn!("Host received unexpected Pong message");
            }
            Message::Error { message } => {
                error!("Client reported error: {}", message);
            }
        }
    }

    Ok(())
}

async fn send_message(socket: &mut tokio::net::TcpStream, message: &Message) -> Result<()> {
    let serialized = serde_json::to_vec(message)?;
    let len = (serialized.len() as u32).to_be_bytes();
    
    socket.write_all(&len).await?;
    socket.write_all(&serialized).await?;
    socket.flush().await?;
    
    Ok(())
}
