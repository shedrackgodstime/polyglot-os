#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

/// UEFI bootloader entry point
#[entry]
fn main() -> Status {
    // Initialize UEFI helpers (logging, etc.)
    uefi::helpers::init().unwrap();
    
    info!("╔════════════════════════════════════════╗");
    info!("║   Polyglot OS UEFI Bootloader v0.1    ║");
    info!("║   Rust Edition 2024                    ║");
    info!("╚════════════════════════════════════════╝");
    info!("");
    
    info!("Boot services initialized");
    info!("Bootloader stage complete");
    info!("");
    
    // For now, just demonstrate the boot process
    // Kernel loading will be implemented in next iteration
    info!("NOTE: Kernel loading not yet implemented");
    info!("The kernel.elf is built and ready in ESP");
    info!("Next step: Implement ELF loader");
    info!("");
    info!("✓ Boot chain working!");
    
    // Show we're alive
    let mut counter = 0;
    loop {
        if counter % 100000000 == 0 {
            info!("Bootloader running... (iteration {})", counter / 100000000);
        }
        counter += 1;
        if counter > 500000000 {
            break;
        }
    }
    
    Status::SUCCESS
}
