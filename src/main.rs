use sysinfo::{System};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    display_cpu_usage(&system);
    display_system_uptime(&system);
    display_memory_usage(&system);
}
//fix
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
    let total_memory_gb = system.total_memory() / 1_048_576; 
    let used_memory_gb = system.used_memory() / 1_048_576; 
    println!("Total memory: {} GB", total_memory_gb);
    println!("Used memory: {} GB", used_memory_gb);
}
