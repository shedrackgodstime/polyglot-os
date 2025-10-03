//! Device drivers subsystem

pub mod timer;
pub mod keyboard;

/// Initialize all device drivers
pub fn init() {
    crate::serial::print("Initializing device drivers...\n");
    
    // Initialize timer (PIT)
    timer::init();
    
    // Initialize keyboard
    keyboard::init();
    
    crate::serial::print("Device drivers initialized.\n");
}
