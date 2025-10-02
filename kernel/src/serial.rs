const COM1_PORT: u16 = 0x3F8;

/// Initializes the COM1 serial port.
pub fn init() {
    unsafe {
        // Disable all interrupts from the serial port
        core::arch::asm!("out dx, al", in("dx") COM1_PORT + 1, in("al") 0x00u8);

        // Enable DLAB (Divisor Latch Access Bit) to set the baud rate
        core::arch::asm!("out dx, al", in("dx") COM1_PORT + 3, in("al") 0x80u8);

        // Set divisor to 3 (lo byte) for 38400 baud (115200 / 3)
        core::arch::asm!("out dx, al", in("dx") COM1_PORT + 0, in("al") 0x03u8);
        // Set divisor to 0 (hi byte)
        core::arch::asm!("out dx, al", in("dx") COM1_PORT + 1, in("al") 0x00u8);

        // Set line control register: 8 bits, no parity, one stop bit
        core::arch::asm!("out dx, al", in("dx") COM1_PORT + 3, in("al") 0x03u8);

        // Enable FIFO, clear them, with 14-byte threshold
        core::arch::asm!("out dx, al", in("dx") COM1_PORT + 2, in("al") 0xC7u8);

        // Enable IRQs, RTS/DSR set
        core::arch::asm!("out dx, al", in("dx") COM1_PORT + 4, in("al") 0x0Bu8);
    }
}

/// Prints a string to the COM1 serial port.
pub fn print(s: &str) {
    for byte in s.bytes() {
        unsafe {
            // Wait until the transmit buffer is empty
            let mut status: u8;
            loop {
                core::arch::asm!("in al, dx", in("dx") COM1_PORT + 5, out("al") status);
                if (status & 0x20) != 0 {
                    break;
                }
            }
            // Write the byte to the serial port
            core::arch::asm!("out dx, al", in("dx") COM1_PORT, in("al") byte);
        }
    }
}
