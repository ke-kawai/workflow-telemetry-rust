use anyhow::Result;
use crate::collectors::{CpuStats, MemoryStats};

/// Markdownレポートを生成
pub fn generate_report(cpu_data: &[CpuStats], memory_data: &[MemoryStats]) -> Result<String> {
    let mut report = String::new();
    
    // ヘッダー
    report.push_str("# Workflow Telemetry Report\n\n");
    
    if cpu_data.is_empty() && memory_data.is_empty() {
        report.push_str("⚠️ No data collected\n\n");
        return Ok(report);
    }
    
    // CPUサマリー
    if !cpu_data.is_empty() {
        let avg_cpu: f64 = cpu_data.iter().map(|s| s.total_load).sum::<f64>() / cpu_data.len() as f64;
        let max_cpu: f64 = cpu_data.iter().map(|s| s.total_load).fold(0.0, f64::max);
        
        report.push_str("## CPU Usage\n\n");
        report.push_str(&format!("- **Average**: {:.2}%\n", avg_cpu));
        report.push_str(&format!("- **Peak**: {:.2}%\n", max_cpu));
        report.push_str(&format!("- **Data Points**: {}\n\n", cpu_data.len()));
    }
    
    // メモリサマリー
    if !memory_data.is_empty() {
        let avg_mem: f64 = memory_data.iter().map(|s| s.used_mb as f64).sum::<f64>() / memory_data.len() as f64;
        let max_mem: u64 = memory_data.iter().map(|s| s.used_mb).max().unwrap_or(0);
        
        report.push_str("## Memory Usage\n\n");
        report.push_str(&format!("- **Average**: {:.0} MB\n", avg_mem));
        report.push_str(&format!("- **Peak**: {} MB\n", max_mem));
        report.push_str(&format!("- **Data Points**: {}\n\n", memory_data.len()));
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
        assert!(report.contains("CPU Usage"));
        assert!(report.contains("Memory Usage"));
        assert!(report.contains("<svg"));
    }

    #[test]
    fn test_generate_report_empty_data() {
        let cpu_data: Vec<CpuStats> = vec![];
        let memory_data: Vec<MemoryStats> = vec![];
        let report = generate_report(&cpu_data, &memory_data).unwrap();
        
        assert!(report.contains("No data collected"));
    }
}
