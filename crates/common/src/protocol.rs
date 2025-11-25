use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Message types exchanged between client and host
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    /// Ping request with timestamp
    Ping { timestamp: DateTime<Utc> },
    
    /// Pong response echoing the original timestamp
    Pong { timestamp: DateTime<Utc> },
    
    /// Test results from client to host
    TestResults { results: TestResults },
    
    /// Error message
    Error { message: String },
}

/// Collection of test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub latency: Option<LatencyResults>,
    pub bandwidth: Option<BandwidthResults>,
    pub video: Option<VideoResults>,
    pub audio: Option<AudioResults>,
}

/// Network latency test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyResults {
    pub min_ms: f64,
    pub max_ms: f64,
    pub avg_ms: f64,
    pub jitter_ms: f64,
    pub packet_loss_percent: f64,
    pub samples: Vec<f64>,
}

/// Bandwidth test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthResults {
    pub download_mbps: f64,
    pub upload_mbps: f64,
}

/// Video codec and performance results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResults {
    pub supported_codecs: Vec<String>,
    pub decode_fps: Option<f64>,
}

/// Audio system results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioResults {
    pub output_devices: Vec<String>,
    pub sample_rate: Option<u32>,
}
