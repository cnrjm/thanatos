extern crate winapi;
use sysinfo::{ProcessExt, System, SystemExt};
use std::ptr;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible};

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

    find_visible_windows();
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
}