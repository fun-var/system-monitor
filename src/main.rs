use std::thread;
use std::time::Duration;
use sysinfo::System;

mod system_info;

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();
        
        system_info::display_cpu_usage(&system);
        system_info::display_system_uptime(&system);
        system_info::display_memory_usage(&system);
        
        system_info::display_temperatures(&system);

        thread::sleep(Duration::from_secs(5));
    }
}