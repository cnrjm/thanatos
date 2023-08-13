use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut s = System::new_all();
    s.refresh_all();
    
    let mut p: Vec<(String, String, u64)> = Vec::new();

    for (pid, process) in s.processes() {
        p.push((pid.to_string(), process.name().to_owned(), process.memory()));
    }

    p.sort_by(|a, b| b.2.cmp(&a.2));

    for (pid, name, memory_bytes) in p.iter() {
        let memory_mb = (*memory_bytes as f64) / (1024.0 * 1024.0);
        println!("PID: {}, Name: {}, Memory: {:.2} MB", pid, name, memory_mb);
    }
}
