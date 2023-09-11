use sysinfo::{ProcessExt, System, SystemExt};
use std::collections::HashMap;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut s = System::new_all();
    s.refresh_all();

    loop {
        let mut process_memory_map: HashMap<String, u64> = HashMap::new();

        for (_pid, process) in s.processes() {
            let process_name = process.name().to_owned();
            let memory_usage = process.memory();

            let entry = process_memory_map.entry(process_name).or_insert(0);
            *entry += memory_usage;
        }

        let mut p: Vec<(String, u64)> = process_memory_map.into_iter().collect();
        p.sort_by(|a, b| b.1.cmp(&a.1));

        println!(
            "\x1B[1m\x1B[4m{:<4} {:<30} {:<10}\x1B[0m",
            "ID", "Name", "Memory (MB)"
        );

        for (index, (name, memory_bytes)) in p.iter().enumerate() {
            let memory_mb = (*memory_bytes as f64) / (1024.0 * 1024.0);
            println!("{:<4} {:<30} {:.2} MB", index + 1, name, memory_mb);
        }

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();

        if input.starts_with("kill ") {
            let index_to_terminate = input["kill ".len()..].parse::<usize>();
            match index_to_terminate {
                Ok(index) if index > 0 && index <= p.len() => {
                    let process_name_to_terminate = &p[index - 1].0;
                    terminate_process(process_name_to_terminate);

                    // Add a delay (e.g., 2 seconds) before reprinting the list
                    sleep(Duration::from_secs(2));
                }
                _ => {
                    println!("Invalid index selected");
                }
            }
        } else if input == "exit" {
            break;
        }
    }
}

fn terminate_process(process_name: &str) {
    // Use the `taskkill` command to terminate the process
    let output = Command::new("taskkill")
        .arg("/F") // Forcefully terminate the process
        .arg("/IM")
        .arg(process_name)
        .output();

    if let Ok(o) = output {
        if o.status.success() {
            println!("Successfully terminated process: {}", process_name);
        } else {
            println!("Failed to terminate process: {}", process_name);
        }
    } else {
        println!("Failed to execute taskkill command");
    }
}
