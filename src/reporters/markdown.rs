use anyhow::Result;
use crate::collectors::{CpuStats, MemoryStats};
use crate::reporters::mermaid::generate_chart;

/// Markdownレポートを生成
pub fn generate_report(cpu_data: &[CpuStats], memory_data: &[MemoryStats]) -> Result<String> {
    let mut report = String::new();
    
    // ヘッダー
    report.push_str("# Workflow Telemetry Report\n\n");
    
    // グラフ
    if !cpu_data.is_empty() || !memory_data.is_empty() {
        report.push_str(&generate_chart(cpu_data, memory_data));
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
        let cpu_data = vec![
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
        
        let memory_data = vec![
            MemoryStats {
                time: 1000,
                usage_percent: 50.0,
                used_mb: 5000,
                total_mb: 10000,
            },
        ];

        let report = generate_report(&cpu_data, &memory_data).unwrap();
        
        assert!(report.contains("# Workflow Telemetry Report"));
        assert!(report.contains("```mermaid"));
        assert!(report.contains("Resource Usage"));
    }

    #[test]
    fn test_generate_report_empty_data() {
        let cpu_data: Vec<CpuStats> = vec![];
        let memory_data: Vec<MemoryStats> = vec![];
        let report = generate_report(&cpu_data, &memory_data).unwrap();
        
        assert!(report.contains("No data collected"));
    }
}
