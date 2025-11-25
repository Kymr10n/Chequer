use chequer_agent::{Host, Client};
use chequer_common::{TestConfig, Message};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_host_client_latency() {
    // Start host in background
    let host = Host::new("127.0.0.1:17777".to_string());
    
    tokio::spawn(async move {
        host.run().await.expect("Host failed");
    });
    
    // Give host time to start
    sleep(Duration::from_millis(100)).await;
    
    // Run client test
    let client = Client::new("127.0.0.1:17777".to_string())
        .with_config(TestConfig {
            latency_samples: 10,
            latency_interval_ms: 5,
            bandwidth_duration_secs: 0,
        });
    
    let results = client.run().await.expect("Client failed");
    
    // Verify results
    assert!(results.latency.is_some());
    let latency = results.latency.unwrap();
    
    assert_eq!(latency.samples.len(), 10);
    assert!(latency.avg_ms > 0.0);
    assert!(latency.min_ms <= latency.avg_ms);
    assert!(latency.max_ms >= latency.avg_ms);
    assert!(latency.jitter_ms >= 0.0);
}

#[tokio::test]
async fn test_message_serialization() {
    use chrono::Utc;
    
    let ping = Message::Ping { timestamp: Utc::now() };
    let serialized = serde_json::to_string(&ping).unwrap();
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    
    match deserialized {
        Message::Ping { .. } => (),
        _ => panic!("Wrong message type"),
    }
}
