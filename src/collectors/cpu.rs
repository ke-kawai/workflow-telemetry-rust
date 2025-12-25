use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// CPU統計データ
#[derive(Debug, Clone, Serialize)]
pub struct CpuStats {
    /// タイムスタンプ (ミリ秒)
    pub time: u64,
    /// 総CPU使用率 (%)
    pub total_load: f64,
    /// ユーザーモードCPU使用率 (%)
    pub user_load: f64,
    /// システムモードCPU使用率 (%)
    pub system_load: f64,
}

/// /proc/stat から読み取った生のCPU時間
#[derive(Debug, Clone, Default)]
struct CpuTime {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
}

impl CpuTime {
    fn total(&self) -> u64 {
        self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
    }

    fn active(&self) -> u64 {
        self.user + self.nice + self.system + self.irq + self.softirq + self.steal
    }

    fn user_time(&self) -> u64 {
        self.user + self.nice
    }

    fn system_time(&self) -> u64 {
        self.system + self.irq + self.softirq
    }
}

/// CPU使用率コレクター
pub struct CpuCollector {
    last_cpu_time: Option<CpuTime>,
}

impl CpuCollector {
    /// 新しいコレクターを作成
    pub fn new() -> Self {
        Self {
            last_cpu_time: None,
        }
    }

    /// CPU統計を収集
    pub fn collect(&mut self) -> Result<CpuStats> {
        let current_time = Self::current_timestamp_ms();
        let current_cpu = Self::read_proc_stat()?;

        let stats = if let Some(last_cpu) = &self.last_cpu_time {
            // 前回からの差分を計算
            let total_delta = current_cpu.total().saturating_sub(last_cpu.total());
            let active_delta = current_cpu.active().saturating_sub(last_cpu.active());
            let user_delta = current_cpu.user_time().saturating_sub(last_cpu.user_time());
            let system_delta = current_cpu.system_time().saturating_sub(last_cpu.system_time());

            let total_load = if total_delta > 0 {
                (active_delta as f64 / total_delta as f64) * 100.0
            } else {
                0.0
            };

            let user_load = if total_delta > 0 {
                (user_delta as f64 / total_delta as f64) * 100.0
            } else {
                0.0
            };

            let system_load = if total_delta > 0 {
                (system_delta as f64 / total_delta as f64) * 100.0
            } else {
                0.0
            };

            CpuStats {
                time: current_time,
                total_load,
                user_load,
                system_load,
            }
        } else {
            // 初回は0を返す
            CpuStats {
                time: current_time,
                total_load: 0.0,
                user_load: 0.0,
                system_load: 0.0,
            }
        };

        // 次回のために現在値を保存
        self.last_cpu_time = Some(current_cpu);

        Ok(stats)
    }

    /// /proc/stat を読み取り
    fn read_proc_stat() -> Result<CpuTime> {
        let content = fs::read_to_string("/proc/stat")
            .context("Failed to read /proc/stat")?;
        Self::parse_proc_stat(&content)
    }

    /// /proc/stat の内容をパース
    fn parse_proc_stat(content: &str) -> Result<CpuTime> {
        // 最初の行 "cpu ..." をパース
        let first_line = content
            .lines()
            .next()
            .context("Empty /proc/stat")?;

        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 9 || parts[0] != "cpu" {
            anyhow::bail!("Invalid /proc/stat format");
        }

        Ok(CpuTime {
            user: parts[1].parse().context("Invalid user time")?,
            nice: parts[2].parse().context("Invalid nice time")?,
            system: parts[3].parse().context("Invalid system time")?,
            idle: parts[4].parse().context("Invalid idle time")?,
            iowait: parts[5].parse().context("Invalid iowait time")?,
            irq: parts[6].parse().context("Invalid irq time")?,
            softirq: parts[7].parse().context("Invalid softirq time")?,
            steal: parts[8].parse().context("Invalid steal time")?,
        })
    }

    /// 現在のタイムスタンプ（ミリ秒）を取得
    fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }
}

impl Default for CpuCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_time_calculations() {
        let cpu_time = CpuTime {
            user: 1000,
            nice: 100,
            system: 500,
            idle: 5000,
            iowait: 200,
            irq: 50,
            softirq: 50,
            steal: 100,
        };

        assert_eq!(cpu_time.total(), 7000);
        assert_eq!(cpu_time.active(), 1800);
        assert_eq!(cpu_time.user_time(), 1100);
        assert_eq!(cpu_time.system_time(), 600);
    }

    #[test]
    fn test_parse_proc_stat() {
        let sample = "cpu  74608 2520 24433 1117073 6176 4054 500 100 0 0";
        let cpu_time = CpuCollector::parse_proc_stat(sample).expect("Failed to parse");
        
        assert_eq!(cpu_time.user, 74608);
        assert_eq!(cpu_time.nice, 2520);
        assert_eq!(cpu_time.system, 24433);
        assert_eq!(cpu_time.idle, 1117073);
    }
}
