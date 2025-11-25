/// Network utility functions and helpers
use anyhow::Result;

/// Validate IP address format
pub fn validate_ip_port(addr: &str) -> Result<()> {
    addr.parse::<std::net::SocketAddr>()
        .map(|_| ())
        .or_else(|_| {
            // Try to parse as hostname:port
            if addr.contains(':') {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Invalid address format. Expected IP:PORT or HOSTNAME:PORT"))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ip_port() {
        assert!(validate_ip_port("127.0.0.1:7777").is_ok());
        assert!(validate_ip_port("192.168.1.100:8080").is_ok());
        assert!(validate_ip_port("localhost:7777").is_ok());
        assert!(validate_ip_port("invalid").is_err());
    }
}
