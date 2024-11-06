#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{CpuExt, SystemExt, ProcessExt, System, PidExt, ProcessStatus};
use tauri::State;
use std::sync::Mutex;
use std::collections::HashMap;

struct AppState {
    sys: Mutex<System>,
    process_cache: Mutex<HashMap<u32, ProcessStaticInfo>>,
}

#[derive(Clone)]
struct ProcessStaticInfo {
    name: String,
    command: String,
    user: String,
}

#[derive(serde::Serialize)]
struct ProcessInfo {
    pid: u32,
    ppid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    status: String,
    user: String,
    command: String,
    threads: Option<u32>,
}

#[derive(serde::Serialize)]
struct SystemStats {
    cpu_usage: Vec<f32>,
    memory_total: u64,
    memory_used: u64,
    memory_free:u64,
    memory_cached: u64,
    swap_space: u64,
    swap_used: u64,
    swap_free: u64,
    uptime: u64,
    load_avg: [f64; 3],
}

#[tauri::command]
async fn get_processes(state: State<'_, AppState>) -> Result<(Vec<ProcessInfo>, SystemStats), String> {
    let processes_data;
    let system_stats;
    
    // Scope for system lock
    {
        let mut sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
        sys.refresh_all();
        
        // Collect all the process data we need while holding sys lock
        processes_data = sys
            .processes()
            .iter()
            .map(|(pid, process)| {
                (
                    pid.as_u32(),
                    process.name().to_string(),
                    process.cmd().to_vec(),
                    process.user_id().map(|uid| uid.to_string()),
                    process.cpu_usage(),
                    process.memory(),
                    process.status(),
                    process.parent().map(|p| p.as_u32()),
                )
            })
            .collect::<Vec<_>>();

        // Get system stats while we still have the sys lock
        let cpu_usage: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        let load_avg = sys.load_average();
        let memory_total = sys.total_memory();
        let memory_used = sys.used_memory();
        let memory_free = memory_total - memory_used;
        let memory_cached = sys.used_swap(); // Estimated

        system_stats = SystemStats {
            cpu_usage,
            memory_total,
            memory_used,
            memory_free,
            memory_cached,
            swap_space: sys.total_swap(),
            swap_used: sys.used_swap(),
            swap_free: sys.free_swap(),
            uptime: sys.uptime(),
            load_avg: [load_avg.one, load_avg.five, load_avg.fifteen],
        };
    } // sys lock is automatically dropped here

    // Now lock the process cache
    let mut process_cache = state.process_cache.lock().map_err(|_| "Failed to lock process cache")?;

    // Build the process info list
    let processes = processes_data
        .into_iter()
        .map(|(pid, name, cmd, user_id, cpu_usage, memory, status, ppid)| {
            let static_info = process_cache.entry(pid).or_insert_with(|| {
                ProcessStaticInfo {
                    name: name.clone(),
                    command: cmd.join(" "),
                    user: user_id.unwrap_or_else(|| "-".to_string()),
                }
            });

            let status_str = match status {
                ProcessStatus::Run => "Running",
                ProcessStatus::Sleep => "Sleeping",
                ProcessStatus::Idle => "Idle",
                _ => "Unknown"
            };

            ProcessInfo {
                pid,
                ppid: ppid.unwrap_or(0),
                name: static_info.name.clone(),
                cpu_usage,
                memory_usage: memory,
                status: status_str.to_string(),
                user: static_info.user.clone(),
                command: static_info.command.clone(),
                threads: None,
            }
        })
        .collect();

    Ok((processes, system_stats))
}

#[tauri::command]
async fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
        Ok(process.kill())
    } else {
        Ok(false)
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(System::new_all()),
            process_cache: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_processes,
            kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}