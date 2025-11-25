/// Terminal visualization utilities for charts and graphs
use std::cmp::min;

/// Generate a sparkline chart from data samples
pub fn sparkline(data: &[f64], width: usize) -> String {
    if data.is_empty() {
        return String::new();
    }

    let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    
    // Downsample if needed
    let samples: Vec<f64> = if data.len() > width {
        data.chunks(data.len() / width)
            .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
            .collect()
    } else {
        data.to_vec()
    };

    let min_val = samples.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_val - min_val;

    if range == 0.0 {
        return blocks[4].to_string().repeat(samples.len());
    }

    samples
        .iter()
        .map(|&val| {
            let normalized = ((val - min_val) / range * 7.0).round() as usize;
            blocks[min(normalized, 7)]
        })
        .collect()
}

/// Generate a horizontal bar chart
#[allow(dead_code)]
pub fn histogram(data: &[f64], bins: usize, width: usize) -> Vec<String> {
    if data.is_empty() || bins == 0 {
        return vec![];
    }

    let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_val - min_val;

    if range == 0.0 {
        return vec!["All values identical".to_string()];
    }

    // Create bins
    let mut bin_counts = vec![0usize; bins];
    for &val in data {
        let bin_idx = (((val - min_val) / range) * (bins as f64 - 0.001)) as usize;
        bin_counts[bin_idx] += 1;
    }

    let max_count = *bin_counts.iter().max().unwrap_or(&1);

    // Generate bars
    bin_counts
        .iter()
        .enumerate()
        .map(|(i, &count)| {
            let bin_start = min_val + (i as f64 * range / bins as f64);
            let bin_end = min_val + ((i + 1) as f64 * range / bins as f64);
            let bar_width = if max_count > 0 {
                (count as f64 / max_count as f64 * width as f64) as usize
            } else {
                0
            };
            
            format!(
                "{:5.1}-{:5.1}ms ┤{}",
                bin_start,
                bin_end,
                "█".repeat(bar_width)
            )
        })
        .collect()
}

/// Calculate percentile value from sorted data
pub fn percentile(data: &[f64], p: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let idx = ((sorted.len() - 1) as f64 * p / 100.0).round() as usize;
    sorted[idx]
}

/// Draw a box with Unicode characters
pub fn draw_box(title: &str, content: Vec<String>, width: usize) -> String {
    let mut output = String::new();
    
    // Top border
    output.push('╔');
    output.push_str(&"═".repeat(width - 2));
    output.push('╗');
    output.push('\n');
    
    // Title
    if !title.is_empty() {
        let padding = (width - 4 - title.len()) / 2;
        output.push('║');
        output.push_str(&" ".repeat(padding));
        output.push_str(title);
        output.push_str(&" ".repeat(width - 2 - padding - title.len()));
        output.push('║');
        output.push('\n');
        
        // Separator
        output.push('╠');
        output.push_str(&"═".repeat(width - 2));
        output.push('╣');
        output.push('\n');
    }
    
    // Content
    for line in content {
        output.push('║');
        output.push(' ');
        output.push_str(&line);
        output.push_str(&" ".repeat(width.saturating_sub(line.len() + 3)));
        output.push('║');
        output.push('\n');
    }
    
    // Bottom border
    output.push('╚');
    output.push_str(&"═".repeat(width - 2));
    output.push('╝');
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let spark = sparkline(&data, 10);
        assert!(!spark.is_empty());
    }

    #[test]
    fn test_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        assert_eq!(percentile(&data, 50.0), 6.0);
        assert_eq!(percentile(&data, 95.0), 10.0);
    }
}
