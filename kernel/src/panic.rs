use core::panic::PanicInfo;

/// Halts the CPU indefinitely.
/// This function is used when the kernel encounters an unrecoverable error.
#[unsafe(no_mangle)]
pub fn hcf() -> ! {
    loop {
        unsafe {
            core::arch::asm!("cli"); // Disable interrupts
            core::arch::asm!("hlt"); // Halt the CPU
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // For now, we just halt the CPU. In the future, we could print panic info to the serial port.
    hcf();
}
