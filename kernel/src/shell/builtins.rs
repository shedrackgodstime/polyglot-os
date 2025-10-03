//! Built-in shell commands

use alloc::string::String;

/// Help command - show available commands
pub fn cmd_help(_args: &[String]) {
    crate::console::println("Available commands:");
    crate::console::println("  help      - Show this help message");
    crate::console::println("  clear     - Clear the screen");
    crate::console::println("  echo      - Print arguments to screen");
    crate::console::println("  mem       - Show memory information");
    crate::console::println("  tasks     - Show task information");
    crate::console::println("  uptime    - Show system uptime");
    crate::console::println("  history   - Show command history");
    crate::console::println("  reboot    - Restart the system");
    crate::console::println("  panic     - Trigger a kernel panic (for testing)");
    crate::console::println("");
}

/// Clear command - clear the screen
pub fn cmd_clear(_args: &[String]) {
    crate::console::clear();
}

/// Echo command - print arguments
pub fn cmd_echo(args: &[String]) {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            crate::console::print(" ");
        }
        crate::console::print(arg);
    }
    crate::console::println("");
}

/// Memory command - show memory information
pub fn cmd_mem(_args: &[String]) {
    let free_memory = crate::memory::physical::free_memory();
    let free_frames = crate::memory::physical::free_frames();
    
    crate::console::println("Memory Information:");
    crate::console::print("  Free memory: ");
    print_size(free_memory);
    crate::console::println("");
    
    crate::console::print("  Free frames: ");
    print_decimal(free_frames as u64);
    crate::console::println("");
    
    crate::console::print("  Frame size: ");
    print_decimal(crate::memory::physical::FRAME_SIZE as u64);
    crate::console::println(" bytes");
    crate::console::println("");
}

/// Tasks command - show task information
pub fn cmd_tasks(_args: &[String]) {
    let (ready_tasks, switches) = crate::task::SCHEDULER.lock().stats();
    
    crate::console::println("Task Information:");
    crate::console::print("  Ready tasks: ");
    print_decimal(ready_tasks as u64);
    crate::console::println("");
    
    crate::console::print("  Context switches: ");
    print_decimal(switches);
    crate::console::println("");
    crate::console::println("");
}

/// Uptime command - show system uptime
pub fn cmd_uptime(_args: &[String]) {
    let ticks = crate::drivers::timer::ticks();
    let seconds = ticks / 100; // 100 Hz timer
    let minutes = seconds / 60;
    let hours = minutes / 60;
    
    crate::console::println("System Uptime:");
    crate::console::print("  ");
    print_decimal(hours);
    crate::console::print("h ");
    print_decimal(minutes % 60);
    crate::console::print("m ");
    print_decimal(seconds % 60);
    crate::console::print("s (");
    print_decimal(ticks);
    crate::console::println(" ticks)");
    crate::console::println("");
}

/// History command - show command history
pub fn cmd_history(_args: &[String], history: &[String]) {
    crate::console::println("Command History:");
    for (i, cmd) in history.iter().enumerate() {
        crate::console::print("  ");
        print_decimal((i + 1) as u64);
        crate::console::print(": ");
        crate::console::println(cmd);
    }
    crate::console::println("");
}

/// Reboot command - restart the system
pub fn cmd_reboot(_args: &[String]) {
    crate::console::println("Rebooting system...");
    crate::console::println("(Note: QEMU might not actually reboot)");
    
    // Try different reboot methods
    unsafe {
        // Method 1: Triple fault (should cause reboot)
        core::arch::asm!("int3; int3; int3");
        
        // Method 2: Reset via keyboard controller
        use x86_64::instructions::port::Port;
        let mut port = Port::new(0x64);
        port.write(0xFE_u8);
        
        // Method 3: ACPI reset (simplified)
        let mut port = Port::new(0xCF9);
        port.write(0x06_u8);
    }
    
    // If we get here, reboot failed
    crate::console::println("Reboot failed. System halted.");
    crate::panic::hcf();
}

/// Panic command - trigger a kernel panic for testing
pub fn cmd_panic(_args: &[String]) {
    crate::console::println("Triggering kernel panic...");
    panic!("User-requested panic from shell");
}

/// Helper function to print decimal numbers
fn print_decimal(value: u64) {
    crate::memory::print_decimal(value);
}

/// Helper function to print sizes
fn print_size(bytes: u64) {
    crate::memory::print_size(bytes);
}
