use sysinfo::System;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

pub async fn fetch_processes(filter: &Option<String>) -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_processes();

    system.processes()
        .values()
        .filter(|process| {
            if let Some(f) = filter {
                process.name().to_lowercase().contains(&f.to_lowercase())
            } else {
                true
            }
        })
        .map(|p| ProcessInfo {
            pid: p.pid().as_u32() as i32,
            name: p.name().to_string(),
            cpu_usage: p.cpu_usage(),
            memory: p.memory(),
        })
        .collect()
}

