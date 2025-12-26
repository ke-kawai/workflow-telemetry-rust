use crate::collectors::{CpuStats, MemoryStats};
use anyhow::Result;
use charts_rs::{LineChart, Series, Color, Box};

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


pub fn generate_combined_chart(cpu_data: &[CpuStats], memory_data: &[MemoryStats]) -> Result<String> {
    if cpu_data.is_empty() || memory_data.is_empty() {
        return Ok(String::new());
    }

    let start_time = cpu_data.first().unwrap().time;

    // X軸ラベル: 開始からの経過秒数
    let x_labels: Vec<String> = cpu_data.iter()
        .map(|s| format!("{}s", (s.time - start_time) / 1000))
        .collect();

    // CPU使用率データ（%）
    let cpu_values: Vec<f32> = cpu_data.iter()
        .map(|s| s.total_load as f32)
        .collect();

    // メモリ使用量データ（GBに変換）
    let memory_values: Vec<f32> = memory_data.iter()
        .map(|s| (s.used_mb as f64 / 1024.0) as f32)
        .collect();

    // メモリの最大容量（GB）
    let max_memory_gb = memory_data.first()
        .map(|s| (s.total_mb as f64 / 1024.0) as f32)
        .unwrap_or(16.0);

    // CPU用のSeries
    let mut cpu_series = Series::new("CPU %".to_string(), cpu_values);
    cpu_series.y_axis_index = 0;
    cpu_series.label_show = false;

    // メモリ用のSeries
    let mut memory_series = Series::new("Memory GB".to_string(), memory_values);
    memory_series.y_axis_index = 1;
    memory_series.label_show = false;

    // LineChartを作成
    let mut chart = LineChart::new_with_theme(
        vec![cpu_series, memory_series],
        x_labels,
        "light"
    );

    chart.title_text = "CPU and Memory Usage".to_string();
    chart.width = 1000.0;
    chart.height = 500.0;

    // 凡例を左寄せにして、タイトルと被らない位置に配置
    chart.legend_align = charts_rs::Align::Left;
    chart.legend_margin = Some(Box {
        top: 10.0,
        left: 20.0,
        right: 10.0,
        bottom: 20.0,
    });

    // エリア塗りつぶしと滑らかなカーブを有効化
    chart.series_fill = true;
    chart.series_smooth = true;

    // シリーズの色を設定（CPU: 赤、メモリ: 緑）
    chart.series_colors = vec![
        Color::from("#FF6B6B"), // CPU: 赤色
        Color::from("#51CF66"), // メモリ: 緑色
    ];

    // 2つ目のY軸を追加
    chart.y_axis_configs.push(chart.y_axis_configs[0].clone());

    // CPU軸（左）の設定：0-100%固定
    chart.y_axis_configs[0].axis_formatter = Some("{c}%".to_string());
    chart.y_axis_configs[0].axis_min = Some(0.0);
    chart.y_axis_configs[0].axis_max = Some(100.0);

    // メモリ軸（右）の設定：0からmax_memory_gb固定
    chart.y_axis_configs[1].axis_formatter = Some("{c}GB".to_string());
    chart.y_axis_configs[1].axis_min = Some(0.0);
    chart.y_axis_configs[1].axis_max = Some(max_memory_gb);

    // SVG生成
    Ok(chart.svg()?)
}
