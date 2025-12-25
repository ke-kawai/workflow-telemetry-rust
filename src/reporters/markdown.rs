use anyhow::Result;
use crate::collectors::{CpuStats, MemoryStats};
use crate::charts::{generate_cpu_chart, generate_memory_chart};
use std::env;
use std::fs;

/// Markdownレポートを生成
pub fn generate_report(cpu_data: &[CpuStats], memory_data: &[MemoryStats]) -> Result<String> {
    let mut report = String::new();
    
    // ヘッダー
    report.push_str("# Workflow Telemetry Report\n\n");
    
    if cpu_data.is_empty() && memory_data.is_empty() {
        report.push_str("⚠️ No data collected\n\n");
        return Ok(report);
    }
    
    // SVGファイルを保存
    let workspace = env::var("GITHUB_WORKSPACE").unwrap_or_else(|_| ".".to_string());
    
    // CPUグラフ（SVG）
    if !cpu_data.is_empty() {
        let cpu_svg = generate_cpu_chart(cpu_data)?;
        let cpu_path = format!("{}/cpu-usage.svg", workspace);
        fs::write(&cpu_path, &cpu_svg)?;
        eprintln!("✅ CPU chart saved to {}", cpu_path);
        
        report.push_str("## CPU Usage\n\n");
        report.push_str("![CPU Usage](./cpu-usage.svg)\n\n");
    }
    
    // メモリグラフ（SVG）
    if !memory_data.is_empty() {
        let mem_svg = generate_memory_chart(memory_data)?;
        let mem_path = format!("{}/memory-usage.svg", workspace);
        fs::write(&mem_path, &mem_svg)?;
        eprintln!("✅ Memory chart saved to {}", mem_path);
        
        report.push_str("## Memory Usage\n\n");
        report.push_str("![Memory Usage](./memory-usage.svg)\n\n");
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
