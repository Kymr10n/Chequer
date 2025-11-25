use serde::{Deserialize, Serialize};

/// Role of the agent instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    Host,
    Client,
}

/// Traffic light status for test results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    /// Good - no issues detected
    Green,
    /// Warning - minor issues detected
    Yellow,
    /// Critical - major issues detected
    Red,
}

impl Status {
    /// Get ANSI color code for terminal output
    pub fn color_code(&self) -> &'static str {
        match self {
            Status::Green => "\x1b[32m",   // Green
            Status::Yellow => "\x1b[33m",  // Yellow
            Status::Red => "\x1b[31m",     // Red
        }
    }

    /// Reset ANSI color
    pub const RESET: &'static str = "\x1b[0m";
}

/// Diagnostic test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub latency_samples: usize,
    pub latency_interval_ms: u64,
    pub bandwidth_duration_secs: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            latency_samples: 100,
            latency_interval_ms: 10,
            bandwidth_duration_secs: 10,
        }
    }
}
