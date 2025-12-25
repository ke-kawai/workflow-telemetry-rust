use crate::collectors::{CpuStats, MemoryStats};

/// Mermaid xyChartを生成（CPU + メモリ）
pub fn generate_chart(cpu_data: &[CpuStats], memory_data: &[MemoryStats]) -> String {
    if cpu_data.is_empty() && memory_data.is_empty() {
        return "```mermaid\n%%{init: {'theme':'base'}}%%\nxychart-beta\n    title \"No data\"\n    x-axis [0]\n    y-axis \"Usage %\" 0 --> 10\n```".to_string();
    }

    let mut chart = String::new();
    chart.push_str("```mermaid\n");
    chart.push_str("%%{init: {'theme': 'dark', 'themeVariables': { 'xyChart': {'backgroundColor': '#0d1117'}}}}%%\n");
    chart.push_str("xychart-beta\n");
    chart.push_str("    title \"Resource Usage\"\n");
    
    // X軸: 開始からの経過秒数
    let start_time = if let Some(s) = cpu_data.first() {
        s.time
    } else if let Some(s) = memory_data.first() {
        s.time
    } else {
        0
    };
    
    let data_count = cpu_data.len().max(memory_data.len());
    chart.push_str("    x-axis [");
    let time_labels: Vec<String> = (0..data_count)
        .filter(|i| i % 5 == 0 || *i == data_count - 1)
        .map(|i| {
            let time = if i < cpu_data.len() {
                cpu_data[i].time
            } else if i < memory_data.len() {
                memory_data[i].time
            } else {
                start_time
            };
            format!("\"{}s\"", (time - start_time) / 1000)
        })
        .collect();
    chart.push_str(&time_labels.join(", "));
    chart.push_str("]\n");
    
    // Y軸: 使用率の最大値を計算
    let max_cpu = cpu_data.iter()
        .map(|s| s.total_load)
        .fold(0.0f64, |a, b| a.max(b));
    let max_memory = memory_data.iter()
        .map(|s| s.usage_percent)
        .fold(0.0f64, |a, b| a.max(b));
    let max_value = max_cpu.max(max_memory).max(10.0).ceil() as i32;
    chart.push_str(&format!("    y-axis \"Usage %\" 0 --> {}\n", max_value));
    
    // CPU使用率（赤、塗りつぶし）
    if !cpu_data.is_empty() {
        chart.push_str("    bar [");
        let cpu_values: Vec<String> = cpu_data.iter()
            .map(|s| format!("{:.1}", s.total_load))
            .collect();
        chart.push_str(&cpu_values.join(", "));
        chart.push_str("]\n");
    }
    
    // メモリ使用率（緑、塗りつぶし）
    if !memory_data.is_empty() {
        chart.push_str("    bar [");
        let memory_values: Vec<String> = memory_data.iter()
            .map(|s| format!("{:.1}", s.usage_percent))
            .collect();
        chart.push_str(&memory_values.join(", "));
        chart.push_str("]\n");
    }
    
    chart.push_str("```\n");
    chart
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_chart() {
        let cpu_data = vec![
            CpuStats {
                time: 1000,
                total_load: 10.5,
                user_load: 6.2,
                system_load: 4.3,
            },
            CpuStats {
                time: 2000,
                total_load: 25.8,
                user_load: 15.1,
                system_load: 10.7,
            },
        ];
        
        let memory_data = vec![
            MemoryStats {
                time: 1000,
                usage_percent: 45.2,
                used_mb: 4520,
                total_mb: 10000,
            },
            MemoryStats {
                time: 2000,
                usage_percent: 48.5,
                used_mb: 4850,
                total_mb: 10000,
            },
        ];

        let chart = generate_chart(&cpu_data, &memory_data);
        
        assert!(chart.contains("```mermaid"));
        assert!(chart.contains("xychart-beta"));
        assert!(chart.contains("Resource Usage"));
        assert!(chart.contains("\"0s\""));
        assert!(chart.contains("10.5"));
        assert!(chart.contains("45.2"));
    }

    #[test]
    fn test_empty_data() {
        let cpu_data: Vec<CpuStats> = vec![];
        let memory_data: Vec<MemoryStats> = vec![];
        let chart = generate_chart(&cpu_data, &memory_data);
        assert!(chart.contains("No data"));
    }
}
