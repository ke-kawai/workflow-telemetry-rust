mod collectors;
mod reporters;
mod charts;

use collectors::{CpuCollector, CpuStats, MemoryCollector, MemoryStats};
use reporters::generate_report;
use std::env;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::Mutex;

fn main() {
    let mut cpu_collector = CpuCollector::new();
    let memory_collector = MemoryCollector::new();
    let cpu_data = Arc::new(Mutex::new(Vec::<CpuStats>::new()));
    let memory_data = Arc::new(Mutex::new(Vec::<MemoryStats>::new()));
    let running = Arc::new(AtomicBool::new(true));

    let interval_secs = env::var("TELEMETRY_INTERVAL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    
    let max_iterations = env::var("TELEMETRY_ITERATIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    eprintln!("Telemetry monitoring started (max {} iterations at {}s intervals)", max_iterations, interval_secs);

    // SIGTERMハンドラー
    let running_clone = running.clone();
    ctrlc::set_handler(move || {
        eprintln!("Received termination signal, stopping...");
        running_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting signal handler");

    let mut count = 0;
    while running.load(Ordering::SeqCst) && count < max_iterations {
        match cpu_collector.collect() {
            Ok(stats) => cpu_data.lock().unwrap().push(stats),
            Err(e) => eprintln!("CPU Error: {}", e),
        }
        match memory_collector.collect() {
            Ok(stats) => memory_data.lock().unwrap().push(stats),
            Err(e) => eprintln!("Memory Error: {}", e),
        }
        count += 1;
        thread::sleep(Duration::from_secs(interval_secs));
    }

    eprintln!("Collected {} data points, generating report...", count);

    let cpu_vec = cpu_data.lock().unwrap().clone();
    let mem_vec = memory_data.lock().unwrap().clone();
    let report = generate_report(&cpu_vec, &mem_vec).expect("Failed to generate report");
    
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
