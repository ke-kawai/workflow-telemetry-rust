use crate::collectors::{CpuStats, MemoryStats};

/// CPUグラフを生成
pub fn generate_cpu_chart(data: &[CpuStats]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let mut chart = String::new();
    chart.push_str("### CPU Usage\n\n");
    chart.push_str("```mermaid\n");
    chart.push_str("xychart-beta\n");
    chart.push_str("    title \"CPU Usage\"\n");
    
    // X軸: 開始からの経過秒数
    let start_time = data.first().unwrap().time;
    chart.push_str("    x-axis [");
    let time_labels: Vec<String> = data.iter()
        .enumerate()
        .filter(|(i, _)| i % 5 == 0 || *i == data.len() - 1)
        .map(|(_, s)| format!("\"{}s\"", (s.time - start_time) / 1000))
        .collect();
    chart.push_str(&time_labels.join(", "));
    chart.push_str("]\n");
    
    // Y軸: 0-100%固定
    chart.push_str("    y-axis \"CPU %\" 0 --> 100\n");
    
    // CPU使用率（赤色の線）
    chart.push_str("    line [");
    let cpu_values: Vec<String> = data.iter()
        .map(|s| format!("{:.1}", s.total_load))
        .collect();
    chart.push_str(&cpu_values.join(", "));
    chart.push_str("]\n");
    
    chart.push_str("```\n\n");
    chart
}

/// メモリグラフを生成
pub fn generate_memory_chart(data: &[MemoryStats]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let total_gb = data.first().unwrap().total_mb as f64 / 1024.0;
    
    let mut chart = String::new();
    chart.push_str("### Memory Usage\n\n");
    chart.push_str("```mermaid\n");
    chart.push_str("xychart-beta\n");
    chart.push_str("    title \"Memory Usage\"\n");
    
    // X軸: 開始からの経過秒数
    let start_time = data.first().unwrap().time;
    chart.push_str("    x-axis [");
    let time_labels: Vec<String> = data.iter()
        .enumerate()
        .filter(|(i, _)| i % 5 == 0 || *i == data.len() - 1)
        .map(|(_, s)| format!("\"{}s\"", (s.time - start_time) / 1000))
        .collect();
    chart.push_str(&time_labels.join(", "));
    chart.push_str("]\n");
    
    // Y軸: 総メモリ量（GB）
    chart.push_str(&format!("    y-axis \"Memory GB\" 0 --> {:.1}\n", total_gb));
    
    // メモリ使用量（緑色の線、GB単位）
    chart.push_str("    line [");
    let memory_values: Vec<String> = data.iter()
        .map(|s| format!("{:.2}", s.used_mb as f64 / 1024.0))
        .collect();
    chart.push_str(&memory_values.join(", "));
    chart.push_str("]\n");
    
    chart.push_str("```\n\n");
    chart
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cpu_chart() {
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

        let chart = generate_cpu_chart(&cpu_data);
        
        assert!(chart.contains("```mermaid"));
        assert!(chart.contains("CPU Usage"));
        assert!(chart.contains("0 --> 100"));
        assert!(chart.contains("10.5"));
    }

    #[test]
    fn test_generate_memory_chart() {
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

        let chart = generate_memory_chart(&memory_data);
        
        assert!(chart.contains("```mermaid"));
        assert!(chart.contains("Memory Usage"));
        assert!(chart.contains("4.41")); // 4520/1024
    }

    #[test]
    fn test_empty_data() {
        let cpu_data: Vec<CpuStats> = vec![];
        let memory_data: Vec<MemoryStats> = vec![];
        let cpu_chart = generate_cpu_chart(&cpu_data);
        let mem_chart = generate_memory_chart(&memory_data);
        assert!(cpu_chart.is_empty());
        assert!(mem_chart.is_empty());
    }
}
