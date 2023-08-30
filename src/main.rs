extern crate winapi;
use sysinfo::{ProcessExt, System, SystemExt};
use std::ptr;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible, PostMessageW, WM_CLOSE};
use std::collections::HashMap;
use std::process::Command;
use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};

fn main() {
    Numrow9Key.bind(||main());
    let mut s = System::new_all();
    s.refresh_all();
    
    let mut process_memory_map: HashMap<String, u64> = HashMap::new();

    for (_pid, process) in s.processes() {
        let process_name = process.name().to_owned();
        let memory_usage = process.memory();

        let entry = process_memory_map.entry(process_name).or_insert(0);
        *entry += memory_usage;
    }

    let mut p: Vec<(String, u64)> = process_memory_map.into_iter().collect();
    p.sort_by(|a, b| b.1.cmp(&a.1));

    for (index, (name, memory_bytes)) in p.iter().enumerate() {
        let memory_mb = (*memory_bytes as f64) / (1024.0 * 1024.0);
        println!("{}. Name: {}, Memory: {:.2} MB", index + 1, name, memory_mb);
    }

    find_visible_windows();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase();

        if input.starts_with("kill ") {
            let index_to_terminate = input["kill ".len()..].parse::<usize>();
            match index_to_terminate {
                Ok(index) if index > 0 && index <= p.len() => {
                    let process_name_to_terminate = &p[index - 1].0;
                    terminate_process(process_name_to_terminate);
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

fn find_visible_windows() {
    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, _l_param: isize) -> winapi::ctypes::c_int {
        let mut buffer = [0u16; 256];
        GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
    
        let window_title = String::from_utf16_lossy(&buffer);
    
        if !window_title.trim().is_empty() && IsWindowVisible(hwnd) != 0 {
            println!("Visible window: {}", window_title.trim());
        }
    
        1 // Continue enumeration
    }

    unsafe {
        EnumWindows(Some(enum_windows_callback), 0);
    }

    inputbot::handle_input_events();
}