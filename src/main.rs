use std::thread;
use std::time::Duration;

use sysinfo::{System};

fn main() {
    let mut system = System::new_all();
    
    loop {
        system.refresh_all();
        
        display_cpu_usage(&system);
        display_system_uptime(&system);
        display_memory_usage(&system);
        thread::sleep(Duration::from_secs((5)));
    }
}

fn display_system_uptime(system: &System) {
    println!("\nSystem Uptime:");
    let uptime = sysinfo::System::uptime();
    println!("Uptime: {} seconds", uptime);
}

fn display_cpu_usage(system: &System) {
    println!("CPU Usage:");
    for (i, cpu) in system.cpus().iter().enumerate() {
        println!("CPU{}: {:.2}%", i, cpu.cpu_usage());
    }
}

fn display_memory_usage(system: &System) {
    println!("\nMemory Usage:");
    let total_memory_gb = system.total_memory() / 1_073_741_824; 
    let used_memory_gb = system.used_memory() / 1_073_741_824; 
    println!("Total memory: {} GB", total_memory_gb);
    println!("Used memory: {} GB", used_memory_gb);
}
