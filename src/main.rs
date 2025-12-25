mod collectors;
mod reporters;

use collectors::{CpuCollector, CpuStats, MemoryCollector, MemoryStats};
use reporters::generate_report;
use std::env;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let mut cpu_collector = CpuCollector::new();
    let memory_collector = MemoryCollector::new();
    let mut cpu_data: Vec<CpuStats> = Vec::new();
    let mut memory_data: Vec<MemoryStats> = Vec::new();

    let iterations = env::var("TELEMETRY_ITERATIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);
    
    let interval_secs = env::var("TELEMETRY_INTERVAL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    eprintln!("Collecting {} iterations at {}s intervals", iterations, interval_secs);

    for i in 0..iterations {
        match cpu_collector.collect() {
            Ok(stats) => cpu_data.push(stats),
            Err(e) => eprintln!("CPU Error: {}", e),
        }
        match memory_collector.collect() {
            Ok(stats) => memory_data.push(stats),
            Err(e) => eprintln!("Memory Error: {}", e),
        }
        if i < iterations - 1 {
            thread::sleep(Duration::from_secs(interval_secs));
        }
    }

    let report = generate_report(&cpu_data, &memory_data).expect("Failed to generate report");
    
    if let Ok(summary_path) = env::var("GITHUB_STEP_SUMMARY") {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&summary_path)
            .expect("Failed to open GITHUB_STEP_SUMMARY");
        writeln!(file, "{}", report).expect("Failed to write report");
        eprintln!("✅ Report written to GitHub Step Summary");
    } else {
        eprintln!("⚠️ GITHUB_STEP_SUMMARY not set");
        std::process::exit(1);
    }
}
