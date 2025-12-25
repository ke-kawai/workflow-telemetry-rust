mod collectors;
mod reporters;
mod charts;

use collectors::{CpuCollector, CpuStats, MemoryCollector, MemoryStats};
use reporters::generate_report;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::Mutex;
use std::io::{self, Write};

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
        let _ = writeln!(io::stderr(), "Received termination signal, stopping...");
        let _ = io::stderr().flush();
        running_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting signal handler");

    let mut count = 0;
    while running.load(Ordering::SeqCst) && count < max_iterations {
        match cpu_collector.collect() {
            Ok(stats) => {
                cpu_data.lock().unwrap().push(stats);
                let _ = writeln!(io::stderr(), "Collected CPU data point {}", count + 1);
                let _ = io::stderr().flush();
            }
            Err(e) => eprintln!("CPU Error: {}", e),
        }
        match memory_collector.collect() {
            Ok(stats) => {
                memory_data.lock().unwrap().push(stats);
            }
            Err(e) => eprintln!("Memory Error: {}", e),
        }
        count += 1;
        
        if count < max_iterations && running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(interval_secs));
        }
    }

    eprintln!("Collected {} data points, generating report...", count);
    let _ = io::stderr().flush();

    let cpu_vec = cpu_data.lock().unwrap().clone();
    let mem_vec = memory_data.lock().unwrap().clone();
    
    eprintln!("CPU data points: {}, Memory data points: {}", cpu_vec.len(), mem_vec.len());
    
    match generate_report(&cpu_vec, &mem_vec) {
        Ok(report) => {
            eprintln!("Report generated successfully, {} bytes", report.len());
            
            if let Ok(summary_path) = env::var("GITHUB_STEP_SUMMARY") {
                eprintln!("Writing to summary file: {}", summary_path);
                match fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&summary_path)
                {
                    Ok(mut file) => {
                        match writeln!(file, "{}", report) {
                            Ok(_) => eprintln!("✅ Report written to GitHub Step Summary"),
                            Err(e) => eprintln!("❌ Failed to write report: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to open summary file: {}", e),
                }
            } else {
                eprintln!("⚠️ GITHUB_STEP_SUMMARY not set");
                println!("{}", report);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to generate report: {}", e);
            std::process::exit(1);
        }
    }
}
