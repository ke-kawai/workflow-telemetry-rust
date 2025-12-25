use anyhow::Result;
use crate::collectors::CpuStats;
use crate::reporters::png::generate_cpu_chart;

/// Markdownレポートを生成
pub fn generate_report(cpu_data: &[CpuStats]) -> Result<String> {
    let mut report = String::new();
    
    // ヘッダー
    report.push_str("# Workflow Telemetry Report\n\n");
    
    // CPU統計サマリー
    if !cpu_data.is_empty() {
        report.push_str("## CPU Metrics\n\n");
        
        // 統計情報を計算
        let total_avg = cpu_data.iter().map(|s| s.total_load).sum::<f64>() / cpu_data.len() as f64;
        let user_avg = cpu_data.iter().map(|s| s.user_load).sum::<f64>() / cpu_data.len() as f64;
        let system_avg = cpu_data.iter().map(|s| s.system_load).sum::<f64>() / cpu_data.len() as f64;
        
        let total_max = cpu_data.iter().map(|s| s.total_load).fold(0.0f64, |a, b| a.max(b));
        let user_max = cpu_data.iter().map(|s| s.user_load).fold(0.0f64, |a, b| a.max(b));
        let system_max = cpu_data.iter().map(|s| s.system_load).fold(0.0f64, |a, b| a.max(b));
        
        report.push_str("### Summary\n\n");
        report.push_str("| Metric | Average | Peak |\n");
        report.push_str("|--------|---------|------|\n");
        report.push_str(&format!("| Total CPU | {:.2}% | {:.2}% |\n", total_avg, total_max));
        report.push_str(&format!("| User CPU | {:.2}% | {:.2}% |\n", user_avg, user_max));
        report.push_str(&format!("| System CPU | {:.2}% | {:.2}% |\n\n", system_avg, system_max));
        
        // グラフを生成
        match generate_cpu_chart(cpu_data) {
            Ok(svg_data_url) => {
                report.push_str("### CPU Usage Over Time\n\n");
                report.push_str(&format!("![CPU Usage]({})\n\n", svg_data_url));
            }
            Err(e) => {
                report.push_str(&format!("⚠️ Failed to generate chart: {}\n\n", e));
            }
        }
    } else {
        report.push_str("⚠️ No CPU data collected\n\n");
    }
    
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_report_with_data() {
        let data = vec![
            CpuStats {
                time: 1000,
                total_load: 10.0,
                user_load: 6.0,
                system_load: 4.0,
            },
            CpuStats {
                time: 2000,
                total_load: 20.0,
                user_load: 12.0,
                system_load: 8.0,
            },
        ];

        let report = generate_report(&data).unwrap();
        
        assert!(report.contains("# Workflow Telemetry Report"));
        assert!(report.contains("## CPU Metrics"));
        assert!(report.contains("Total CPU"));
        assert!(report.contains("15.00%")); // average
        assert!(report.contains("20.00%")); // peak
    }

    #[test]
    fn test_generate_report_empty_data() {
        let data: Vec<CpuStats> = vec![];
        let report = generate_report(&data).unwrap();
        
        assert!(report.contains("No CPU data collected"));
    }
}
