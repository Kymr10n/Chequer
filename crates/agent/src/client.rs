use anyhow::{Context, Result};
use chequer_common::{Message, LatencyResults, TestResults, TestConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, debug};
use chrono::Utc;
use std::time::Instant;
use std::os::unix::io::AsRawFd;
use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

/// Client agent that connects to host and runs diagnostics
pub struct Client {
    host_addr: String,
    config: TestConfig,
}

impl Client {
    /// Create a new client instance
    pub fn new(host_addr: String) -> Self {
        Self {
            host_addr,
            config: TestConfig::default(),
        }
    }

    /// Configure test parameters
    pub fn with_config(mut self, config: TestConfig) -> Self {
        self.config = config;
        self
    }

    /// Connect to host and run all diagnostics
    pub async fn run(&self) -> Result<TestResults> {
        info!("Connecting to host at {}...", self.host_addr);
        
        let mut socket = TcpStream::connect(&self.host_addr).await?;
        socket.set_nodelay(true)?;
        
        // Disable TCP delayed ACK on Linux (TCP_QUICKACK)
        #[cfg(target_os = "linux")]
        unsafe {
            let fd = socket.as_raw_fd();
            let tcp_quickack: libc::c_int = 1;
            libc::setsockopt(
                fd,
                libc::IPPROTO_TCP,
                libc::TCP_QUICKACK,
                &tcp_quickack as *const _ as *const libc::c_void,
                std::mem::size_of_val(&tcp_quickack) as libc::socklen_t,
            );
        }
        
        info!("Connected successfully");

        // Run latency test
        let latency = self.run_latency_test(&mut socket).await?;
        
        // Send results to host
        let results = TestResults {
            latency: Some(latency),
            bandwidth: None,
            video: None,
            audio: None,
        };

        self.send_results(&mut socket, &results).await?;
        
        Ok(results)
    }

    async fn run_latency_test(&self, socket: &mut TcpStream) -> Result<LatencyResults> {
        info!("Running latency test ({} samples)...", self.config.latency_samples);
        
        let mut samples = Vec::with_capacity(self.config.latency_samples);
        
        // Pure measurement loop - NO UI rendering during measurement
        for i in 0..self.config.latency_samples {
            let timestamp = Utc::now();
            let ping = Message::Ping { timestamp };
            
            send_message(socket, &ping).await?;
            
            // Re-enable TCP_QUICKACK before each receive (it gets cleared after every recv)
            #[cfg(target_os = "linux")]
            unsafe {
                use std::os::unix::io::AsRawFd;
                let fd = socket.as_raw_fd();
                let tcp_quickack: libc::c_int = 1;
                libc::setsockopt(
                    fd,
                    libc::IPPROTO_TCP,
                    libc::TCP_QUICKACK,
                    &tcp_quickack as *const _ as *const libc::c_void,
                    std::mem::size_of_val(&tcp_quickack) as libc::socklen_t,
                );
            }
            
            let start = Instant::now();
            let response = receive_message(socket).await?;
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            
            match response {
                Message::Pong { timestamp: recv_timestamp } => {
                    if recv_timestamp == timestamp {
                        samples.push(elapsed);
                        debug!("Sample {}/{}: {:.2}ms", i + 1, self.config.latency_samples, elapsed);
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!("Expected Pong, got unexpected message"));
                }
            }
            
            if i < self.config.latency_samples - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(self.config.latency_interval_ms)).await;
            }
        }
        
        // Show completion (after all measurements are done)
        let mut stdout = stdout();
        stdout.execute(SetForegroundColor(Color::Green))?;
        println!("\n│ Progress: [████████████████████████████████████████] 100%");
        stdout.execute(SetForegroundColor(Color::Cyan))?;
        println!("└───────────────────────────────────────────────────────┘");
        stdout.execute(ResetColor)?;
        
        // Calculate statistics
        let min_ms = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_ms = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg_ms = samples.iter().sum::<f64>() / samples.len() as f64;
        
        // Calculate jitter (standard deviation)
        let variance = samples.iter()
            .map(|&x| (x - avg_ms).powi(2))
            .sum::<f64>() / samples.len() as f64;
        let jitter_ms = variance.sqrt();
        
        info!("Latency test complete - Min: {:.2}ms, Max: {:.2}ms, Avg: {:.2}ms, Jitter: {:.2}ms",
              min_ms, max_ms, avg_ms, jitter_ms);
        
        Ok(LatencyResults {
            min_ms,
            max_ms,
            avg_ms,
            jitter_ms,
            packet_loss_percent: 0.0, // No packet loss in TCP
            samples,
        })
    }

    async fn send_results(&self, socket: &mut TcpStream, results: &TestResults) -> Result<()> {
        info!("Sending results to host");
        let message = Message::TestResults { 
            results: results.clone() 
        };
        send_message(socket, &message).await
    }
}

async fn send_message(socket: &mut TcpStream, message: &Message) -> Result<()> {
    let serialized = serde_json::to_vec(message)?;
    let len = (serialized.len() as u32).to_be_bytes();
    
    socket.write_all(&len).await?;
    socket.write_all(&serialized).await?;
    socket.flush().await?;
    
    Ok(())
}

async fn receive_message(socket: &mut TcpStream) -> Result<Message> {
    let mut len_buf = [0u8; 4];
    socket.read_exact(&mut len_buf).await?;
    let msg_len = u32::from_be_bytes(len_buf) as usize;
    
    let mut buffer = vec![0u8; msg_len];
    socket.read_exact(&mut buffer).await?;
    
    let message: Message = serde_json::from_slice(&buffer)
        .context("Failed to deserialize message")?;
    
    Ok(message)
}
