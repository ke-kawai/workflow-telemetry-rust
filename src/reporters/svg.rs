use anyhow::{Context, Result};
use plotters::prelude::*;

use crate::collectors::CpuStats;

/// SVGグラフを生成してBase64エンコードされた文字列を返す
pub fn generate_cpu_chart(data: &[CpuStats]) -> Result<String> {
    if data.is_empty() {
        anyhow::bail!("No data to plot");
    }

    // SVGバッファ
    let mut buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut buffer, (800, 400)).into_drawing_area();
        root.fill(&WHITE)
            .context("Failed to fill background")?;

        // タイムスタンプの範囲を計算
        let min_time = data.first().unwrap().time;
        let max_time = data.last().unwrap().time;
        let time_range = if max_time > min_time {
            min_time..max_time
        } else {
            min_time..(min_time + 1000) // 最低1秒の範囲を確保
        };

        // CPU使用率の最大値を計算（0-100%）
        let max_cpu = data
            .iter()
            .map(|s| s.total_load)
            .fold(0.0f64, |a, b| a.max(b))
            .max(10.0); // 最低10%の範囲を確保

        let mut chart = ChartBuilder::on(&root)
            .caption("CPU Usage (%)", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(time_range, 0.0..max_cpu.ceil())?;

        chart
            .configure_mesh()
            .x_desc("Time (ms)")
            .y_desc("CPU %")
            .draw()?;

        // Total CPU (青)
        chart
            .draw_series(LineSeries::new(
                data.iter().map(|s| (s.time, s.total_load)),
                &BLUE,
            ))?
            .label("Total")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

        // User CPU (緑)
        chart
            .draw_series(LineSeries::new(
                data.iter().map(|s| (s.time, s.user_load)),
                &GREEN,
            ))?
            .label("User")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

        // System CPU (赤)
        chart
            .draw_series(LineSeries::new(
                data.iter().map(|s| (s.time, s.system_load)),
                &RED,
            ))?
            .label("System")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
    }

    // Base64エンコード
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, buffer.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{}", encoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cpu_chart() {
        let data = vec![
            CpuStats {
                time: 1000,
                total_load: 10.0,
                user_load: 6.0,
                system_load: 4.0,
            },
            CpuStats {
                time: 2000,
                total_load: 25.0,
                user_load: 15.0,
                system_load: 10.0,
            },
            CpuStats {
                time: 3000,
                total_load: 15.0,
                user_load: 9.0,
                system_load: 6.0,
            },
        ];

        let result = generate_cpu_chart(&data);
        assert!(result.is_ok());

        let svg = result.unwrap();
        assert!(svg.starts_with("data:image/svg+xml;base64,"));
        assert!(svg.len() > 100);
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<CpuStats> = vec![];
        let result = generate_cpu_chart(&data);
        assert!(result.is_err());
    }
}
