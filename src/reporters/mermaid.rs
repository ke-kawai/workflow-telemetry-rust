use crate::collectors::CpuStats;

/// Mermaid xyChartを生成
pub fn generate_cpu_chart(data: &[CpuStats]) -> String {
    if data.is_empty() {
        return "```mermaid\n%%{init: {'theme':'base'}}%%\nxychart-beta\n    title \"No data\"\n    x-axis [0]\n    y-axis \"Usage %\" 0 --> 10\n```".to_string();
    }

    let mut chart = String::new();
    chart.push_str("```mermaid\n");
    chart.push_str("%%{init: {'theme':'base'}}%%\n");
    chart.push_str("xychart-beta\n");
    chart.push_str("    title \"Resource Usage\"\n");
    
    // X軸: 開始からの経過秒数
    let start_time = data.first().unwrap().time;
    chart.push_str("    x-axis [");
    let time_labels: Vec<String> = data.iter()
        .enumerate()
        .filter(|(i, _)| i % 5 == 0 || *i == data.len() - 1) // 5個おきにラベル表示
        .map(|(_, s)| format!("\"{}s\"", (s.time - start_time) / 1000))
        .collect();
    chart.push_str(&time_labels.join(", "));
    chart.push_str("]\n");
    
    // Y軸: 使用率の最大値を計算
    let max_value = data.iter()
        .map(|s| s.total_load)
        .fold(0.0f64, |a, b| a.max(b))
        .max(10.0)
        .ceil() as i32;
    chart.push_str(&format!("    y-axis \"Usage %\" 0 --> {}\n", max_value));
    
    // CPU Total
    chart.push_str("    line [");
    let total_data: Vec<String> = data.iter()
        .map(|s| format!("{:.1}", s.total_load))
        .collect();
    chart.push_str(&total_data.join(", "));
    chart.push_str("]\n");
    
    // CPU User
    chart.push_str("    line [");
    let user_data: Vec<String> = data.iter()
        .map(|s| format!("{:.1}", s.user_load))
        .collect();
    chart.push_str(&user_data.join(", "));
    chart.push_str("]\n");
    
    // CPU System
    chart.push_str("    line [");
    let system_data: Vec<String> = data.iter()
        .map(|s| format!("{:.1}", s.system_load))
        .collect();
    chart.push_str(&system_data.join(", "));
    chart.push_str("]\n");
    
    chart.push_str("```\n");
    chart
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mermaid_chart() {
        let data = vec![
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
            CpuStats {
                time: 3000,
                total_load: 15.3,
                user_load: 9.5,
                system_load: 5.8,
            },
        ];

        let chart = generate_cpu_chart(&data);
        
        assert!(chart.contains("```mermaid"));
        assert!(chart.contains("xychart-beta"));
        assert!(chart.contains("Resource Usage"));
        assert!(chart.contains("\"0s\""));
        assert!(chart.contains("10.5"));
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<CpuStats> = vec![];
        let chart = generate_cpu_chart(&data);
        assert!(chart.contains("No data"));
    }
}
