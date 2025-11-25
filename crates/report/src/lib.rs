use chequer_common::{Status, TestResults, LatencyResults};
use serde::{Deserialize, Serialize};

/// Diagnostic report with analyzed results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub overall_status: Status,
    pub latency_status: Option<Status>,
    pub bandwidth_status: Option<Status>,
    pub video_status: Option<Status>,
    pub audio_status: Option<Status>,
    pub recommendations: Vec<String>,
    pub raw_results: TestResults,
}

impl DiagnosticReport {
    /// Generate report from test results
    pub fn from_results(results: TestResults) -> Self {
        let mut recommendations = Vec::new();
        
        // Analyze latency
        let latency_status = results.latency.as_ref().map(|lat| {
            analyze_latency(lat, &mut recommendations)
        });

        // TODO: Analyze bandwidth, video, audio
        let bandwidth_status = None;
        let video_status = None;
        let audio_status = None;

        // Determine overall status (worst of all tests)
        let overall_status = [latency_status, bandwidth_status, video_status, audio_status]
            .iter()
            .filter_map(|s| *s)
            .max_by_key(|s| match s {
                Status::Red => 3,
                Status::Yellow => 2,
                Status::Green => 1,
            })
            .unwrap_or(Status::Green);

        Self {
            overall_status,
            latency_status,
            bandwidth_status,
            video_status,
            audio_status,
            recommendations,
            raw_results: results,
        }
    }

    /// Print report to terminal with colors
    pub fn print_terminal(&self) {
        println!("\n{:=^60}", " CHEQUER DIAGNOSTIC REPORT ");
        
        println!("\n{} Overall Status: {}{:?}{}", 
            self.overall_status.color_code(),
            self.overall_status.color_code(),
            self.overall_status,
            Status::RESET
        );

        if let Some(status) = self.latency_status {
            println!("\n{} Network Latency: {}{:?}{}", 
                status.color_code(),
                status.color_code(),
                status,
                Status::RESET
            );
            
            if let Some(lat) = &self.raw_results.latency {
                println!("  Min: {:.2}ms | Max: {:.2}ms | Avg: {:.2}ms | Jitter: {:.2}ms",
                    lat.min_ms, lat.max_ms, lat.avg_ms, lat.jitter_ms);
            }
        }

        if !self.recommendations.is_empty() {
            println!("\n{} Recommendations:", "\x1b[36m");
            for rec in &self.recommendations {
                println!("  • {}", rec);
            }
            print!("{}", Status::RESET);
        }

        println!("\n{:=^60}\n", "");
    }

    /// Export report as JSON
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

fn analyze_latency(lat: &LatencyResults, recommendations: &mut Vec<String>) -> Status {
    if lat.avg_ms > 50.0 {
        recommendations.push("High latency detected. Check network congestion.".to_string());
        recommendations.push("Consider using wired connection instead of WiFi.".to_string());
        Status::Red
    } else if lat.avg_ms > 20.0 || lat.jitter_ms > 10.0 {
        recommendations.push("Moderate latency or jitter detected.".to_string());
        if lat.jitter_ms > 10.0 {
            recommendations.push("Enable QoS on your router for smoother streaming.".to_string());
        }
        Status::Yellow
    } else {
        Status::Green
    }
}
