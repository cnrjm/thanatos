use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
fn main() {
    let mut s = System::new_all();
    s.refresh_all();

    for(pid, process) in s.processes() {
        println!("{} {}", process.name(), process.memory())
    }
}