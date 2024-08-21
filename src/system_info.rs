use sysinfo::{System};

pub fn display_system_uptime(system: &System) {
    println!("\nSystem Uptime:");
    
    let uptime = sysinfo::System::uptime();
    println!("Uptime: {} seconds", uptime);
}

pub fn display_cpu_usage(system: &System) {
    println!("CPU Usage:");

    for (i, cpu) in system.cpus().iter().enumerate() {
        println!("CPU{}: {:.2}%", i, cpu.cpu_usage());
    }
}

pub fn display_memory_usage(system: &System) {
    println!("\nMemory Usage:");

    let total_memory_gb = system.total_memory() / 1_073_741_824; 
    let used_memory_gb = system.used_memory() / 1_073_741_824; 

    println!("Total memory: {} GB", total_memory_gb);
    println!("Used memory: {} GB", used_memory_gb);
}