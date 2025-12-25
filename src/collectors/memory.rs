use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// メモリ統計データ
#[derive(Debug, Clone, Serialize)]
pub struct MemoryStats {
    /// タイムスタンプ (ミリ秒)
    pub time: u64,
    /// メモリ使用率 (%)
    pub usage_percent: f64,
    /// 使用中メモリ (MB)
    pub used_mb: u64,
    /// 総メモリ (MB)
    pub total_mb: u64,
}

pub struct MemoryCollector;

impl MemoryCollector {
    pub fn new() -> Self {
        Self
    }

    /// メモリ使用状況を取得
    pub fn collect(&self) -> Result<MemoryStats> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_millis() as u64;

        let content = fs::read_to_string("/proc/meminfo")
            .context("Failed to read /proc/meminfo")?;

        let mut total_kb = 0u64;
        let mut available_kb = 0u64;

        for line in content.lines() {
            if let Some(value) = line.strip_prefix("MemTotal:") {
                total_kb = value.trim().split_whitespace().next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if let Some(value) = line.strip_prefix("MemAvailable:") {
                available_kb = value.trim().split_whitespace().next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            }
        }

        let used_kb = total_kb.saturating_sub(available_kb);
        let total_mb = total_kb / 1024;
        let used_mb = used_kb / 1024;
        let usage_percent = if total_kb > 0 {
            (used_kb as f64 / total_kb as f64) * 100.0
        } else {
            0.0
        };

        Ok(MemoryStats {
            time: now,
            usage_percent,
            used_mb,
            total_mb,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_memory() {
        let collector = MemoryCollector::new();
        let stats = collector.collect();
        
        if let Ok(stats) = stats {
            assert!(stats.time > 0);
            assert!(stats.usage_percent >= 0.0);
            assert!(stats.usage_percent <= 100.0);
            assert!(stats.total_mb > 0);
        }
    }
}
