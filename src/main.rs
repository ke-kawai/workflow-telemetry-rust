mod collectors;
mod reporters;
mod charts;

use collectors::{CpuCollector, CpuStats, MemoryCollector, MemoryStats};
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
    let cpu_data_clone = cpu_data.clone();
    let memory_data_clone = memory_data.clone();
    
    ctrlc::set_handler(move || {
        let _ = writeln!(io::stderr(), "Received termination signal, saving data...");
        let _ = io::stderr().flush();
        running_clone.store(false, Ordering::SeqCst);
        
        // シグナル受信時にJSON保存
        save_json_data(&cpu_data_clone.lock().unwrap(), &memory_data_clone.lock().unwrap());
        std::process::exit(0);
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

    eprintln!("Collected {} data points, saving data...", count);
    let _ = io::stderr().flush();

    let cpu_vec = cpu_data.lock().unwrap().clone();
    let mem_vec = memory_data.lock().unwrap().clone();
    
    save_json_data(&cpu_vec, &mem_vec);
}

fn save_json_data(cpu_data: &[CpuStats], memory_data: &[MemoryStats]) {
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct TelemetryData {
        cpu: Vec<CpuStats>,
        memory: Vec<MemoryStats>,
    }
    
    let data = TelemetryData {
        cpu: cpu_data.to_vec(),
        memory: memory_data.to_vec(),
    };
    
    match serde_json::to_string_pretty(&data) {
        Ok(json) => {
            if let Err(e) = fs::write("/tmp/telemetry_data.json", &json) {
                eprintln!("Failed to write JSON: {}", e);
            } else {
                eprintln!("✅ Data saved to /tmp/telemetry_data.json");
            }
        }
        Err(e) => eprintln!("Failed to serialize JSON: {}", e),
    }
}
