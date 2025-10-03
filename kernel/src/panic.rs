use core::panic::PanicInfo;

/// Simple function to write a byte to serial port COM1 (0x3F8)
fn serial_write_byte(c: u8) {
    const COM1: u16 = 0x3F8;
    unsafe {
        while (core::ptr::read_volatile((COM1 + 5) as *const u8) & 0x20) == 0 {} // Wait for transmit buffer empty
        core::ptr::write_volatile(COM1 as *mut u8, c);
    }
}

/// Simple function to write a string to serial port
fn serial_print(s: &str) {
    for byte in s.as_bytes() {
        serial_write_byte(*byte);
    }
}

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
fn panic(info: &PanicInfo) -> ! {
    // Print panic information to serial port for debugging
    serial_print("\n!!! KERNEL PANIC !!!\n");
    serial_print("Message: Panic occurred\n");

    // Try to print location information if available
    if let Some(location) = info.location() {
        serial_print("Location: ");
        serial_print(location.file());
        serial_print(":");
        // Convert line number to string manually to avoid heap allocation
        let mut line = location.line();
        if line == 0 {
            serial_write_byte(b'0');
        } else {
            let mut buffer = [0u8; 10];
            let mut i = 0;
            while line > 0 && i < 10 {
                buffer[i] = b'0' + (line % 10) as u8;
                line /= 10;
                i += 1;
            }
            while i > 0 {
                i -= 1;
                serial_write_byte(buffer[i]);
            }
        }
        serial_print("\n");
    }

    serial_print("Halting CPU...\n");
    hcf();
}
