//! Programmable Interval Timer (PIT) driver

use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

const PIT_FREQUENCY: u32 = 1193182; // PIT base frequency in Hz
const TARGET_FREQUENCY: u32 = 100;  // 100 Hz = 10ms intervals

static TICK_COUNT: Mutex<u64> = Mutex::new(0);

/// Initialize the PIT timer
pub fn init() {
    let divisor = PIT_FREQUENCY / TARGET_FREQUENCY;
    
    crate::serial::print("Initializing PIT timer at ");
    crate::memory::print_decimal(TARGET_FREQUENCY as u64);
    crate::serial::print(" Hz...\n");
    
    unsafe {
        // Configure PIT channel 0 for square wave mode
        let mut cmd_port = Port::new(0x43);
        let mut data_port = Port::new(0x40);
        
        // Command: Channel 0, lobyte/hibyte, square wave mode
        cmd_port.write(0x36u8);
        
        // Set divisor (low byte first, then high byte)
        data_port.write((divisor & 0xFF) as u8);
        data_port.write((divisor >> 8) as u8);
    }
    
    // Enable timer interrupt (IRQ 0 -> IRQ 32)
    // TODO: Temporarily disabled to avoid double fault
    // enable_timer_interrupt();
    
    crate::serial::print("PIT timer initialized.\n");
}

/// Enable timer interrupt in PIC
fn enable_timer_interrupt() {
    unsafe {
        let mut pic1_data = Port::new(0x21);
        let mask: u8 = pic1_data.read();
        // Clear bit 0 to enable IRQ 0 (timer)
        pic1_data.write(mask & !0x01);
    }
}

/// Timer interrupt handler (IRQ 0 -> IRQ 32)
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Increment tick counter
    *TICK_COUNT.lock() += 1;
    
    let ticks = *TICK_COUNT.lock();
    
    // Print a dot every second (100 ticks)
    if ticks % 100 == 0 {
        crate::serial::print(".");
        if ticks % 1000 == 0 {
            crate::serial::print(" ");
            crate::memory::print_decimal(ticks / 100);
            crate::serial::print("s\n");
        }
    }
    
    // Notify scheduler of timer tick
    crate::task::scheduler::timer_tick();
    
    // Send EOI to PIC
    crate::interrupts::pic::send_eoi(0);
}

/// Get current tick count
pub fn ticks() -> u64 {
    *TICK_COUNT.lock()
}

/// Sleep for approximately the given number of milliseconds
pub fn sleep_ms(ms: u64) {
    let target_ticks = ticks() + (ms * TARGET_FREQUENCY as u64) / 1000;
    while ticks() < target_ticks {
        x86_64::instructions::hlt();
    }
}

/// Get uptime in seconds
pub fn uptime_seconds() -> u64 {
    ticks() / TARGET_FREQUENCY as u64
}
