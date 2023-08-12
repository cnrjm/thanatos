use sysinfo::{ProcessExt, System, SystemExt};
fn main() {
    let mut s = System::new_all();
    s.refresh_all();
    
    let mut p: Vec<(String, u64)> = Vec::new();

    for (_pid, process) in s.processes() {
        p.push((process.name().to_owned(), process.memory()));
    }

    p.sort_by(|a, b| b.1.cmp(&a.1));

    for (name, memory) in p.iter() {
        println!("Name: {}, Memory: {}", name, memory);
    }
}