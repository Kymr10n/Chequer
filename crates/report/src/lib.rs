use chequer_common::{Status, TestResults, LatencyResults};
use serde::{Deserialize, Serialize};
use crossterm::style::{Color, Stylize};

mod visualization;
use visualization::{sparkline, percentile, draw_box};

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
        let width = 62;
        
        // Build content lines
        let mut content = Vec::new();
        
        // Overall status with emoji
        let status_emoji = match self.overall_status {
            Status::Green => "🟢",
            Status::Yellow => "🟡",
            Status::Red => "🔴",
        };
        
        let status_line = format!(
            "{} Overall Status: {}",
            status_emoji,
            format!("{:?}", self.overall_status)
                .with(match self.overall_status {
                    Status::Green => Color::Green,
                    Status::Yellow => Color::Yellow,
                    Status::Red => Color::Red,
                })
        );
        content.push(status_line);
        content.push(String::new());

        // Latency section with visualization
        if let Some(lat) = &self.raw_results.latency {
            let status = self.latency_status.unwrap_or(Status::Green);
            let status_emoji = match status {
                Status::Green => "🟢",
                Status::Yellow => "🟡",
                Status::Red => "🔴",
            };
            
            content.push(format!(
                "{} Network Latency: {}",
                status_emoji,
                format!("{:?}", status)
                    .with(match status {
                        Status::Green => Color::Green,
                        Status::Yellow => Color::Yellow,
                        Status::Red => Color::Red,
                    })
            ));
            content.push(String::new());

            // Create sparkline visualization
            let spark = sparkline(&lat.samples, 50);
            let p50 = percentile(&lat.samples, 50.0);
            let p95 = percentile(&lat.samples, 95.0);
            
            // Draw simple histogram/distribution
            content.push(format!("  {:>5.1} ┤{}", lat.min_ms, 
                spark.chars().take(50).collect::<String>()));
            content.push(format!("  {:>5.1} ┤{}", lat.avg_ms,
                " ".repeat(50)));
            content.push(format!("  {:>5.1} ┤", lat.max_ms));
            content.push(format!("         └{}", "─".repeat(50)));
            content.push(format!("           {}  {}  {}  {}  {}",
                "Min".with(Color::Cyan),
                "Avg".with(Color::Cyan),
                "P50".with(Color::Cyan),
                "P95".with(Color::Cyan),
                "Max".with(Color::Cyan)));
            content.push(String::new());

            // Statistics
            content.push(format!(
                "  Min: {:.2}ms │ Max: {:.2}ms │ Avg: {:.2}ms",
                lat.min_ms, lat.max_ms, lat.avg_ms
            ));
            content.push(format!(
                "  P50: {:.2}ms │ P95: {:.2}ms │ Jitter: {:.2}ms",
                p50, p95, lat.jitter_ms
            ));
            content.push(format!(
                "  Samples: {} │ Loss: {:.1}%",
                lat.samples.len(), lat.packet_loss_percent
            ));
            content.push(String::new());
        }

        // Recommendations
        if !self.recommendations.is_empty() {
            content.push(format!("{} Recommendations:", "💡".with(Color::Blue)));
            for rec in &self.recommendations {
                content.push(format!("  {} {}", "✓".with(Color::Green), rec));
            }
            content.push(String::new());
        }

        // Print the fancy box
        println!("\n{}\n", draw_box("CHEQUER DIAGNOSTIC REPORT", content, width));
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
