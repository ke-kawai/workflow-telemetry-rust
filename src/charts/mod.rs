use crate::collectors::{CpuStats, MemoryStats};
use anyhow::Result;
use charts_rs::{LineChart, Series, svg_to_png};

pub fn generate_cpu_chart(data: &[CpuStats]) -> Result<String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let start_time = data.first().unwrap().time;
    
    // X軸ラベル: 開始からの経過秒数
    let x_labels: Vec<String> = data.iter()
        .map(|s| format!("{}s", (s.time - start_time) / 1000))
        .collect();
    
    // CPU使用率データ
    let cpu_values: Vec<f32> = data.iter()
        .map(|s| s.total_load as f32)
        .collect();
    
    let mut chart = LineChart::new_with_theme(vec![
        Series::new("CPU %".to_string(), cpu_values)
    ], x_labels, "light");
    
    chart.title_text = "CPU Usage".to_string();
    chart.width = 800.0;
    chart.height = 400.0;
    
    Ok(chart.svg()?)
}

pub fn generate_cpu_chart_png(data: &[CpuStats]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let svg = generate_cpu_chart(data)?;
    Ok(svg_to_png(&svg)?)
}

pub fn generate_memory_chart(data: &[MemoryStats]) -> Result<String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let start_time = data.first().unwrap().time;
    
    // X軸ラベル
    let x_labels: Vec<String> = data.iter()
        .map(|s| format!("{}s", (s.time - start_time) / 1000))
        .collect();
    
    // メモリ使用量（GB）
    let memory_values: Vec<f32> = data.iter()
        .map(|s| (s.used_mb as f64 / 1024.0) as f32)
        .collect();
    
    let mut chart = LineChart::new_with_theme(vec![
        Series::new("Memory GB".to_string(), memory_values)
    ], x_labels, "light");
    
    chart.title_text = "Memory Usage".to_string();
    chart.width = 800.0;
    chart.height = 400.0;
    
    Ok(chart.svg()?)
}

pub fn generate_memory_chart_png(data: &[MemoryStats]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let svg = generate_memory_chart(data)?;
    Ok(svg_to_png(&svg)?)
}
