use anyhow::Result;
use crate::collectors::CpuStats;
use crate::reporters::mermaid::generate_cpu_chart;

/// Markdownレポートを生成
pub fn generate_report(cpu_data: &[CpuStats]) -> Result<String> {
    let mut report = String::new();
    
    // ヘッダー
    report.push_str("# Workflow Telemetry Report\n\n");
    
    // グラフ
    if !cpu_data.is_empty() {
        report.push_str(&generate_cpu_chart(cpu_data));
        report.push_str("\n");
    } else {
        report.push_str("⚠️ No data collected\n\n");
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
        assert!(report.contains("```mermaid"));
        assert!(report.contains("Resource Usage"));
    }

    #[test]
    fn test_generate_report_empty_data() {
        let data: Vec<CpuStats> = vec![];
        let report = generate_report(&data).unwrap();
        
        assert!(report.contains("No data collected"));
    }
}
