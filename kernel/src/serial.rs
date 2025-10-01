/// Serial port driver for COM1
/// Provides basic serial output functionality for debugging

const COM1_PORT: u16 = 0x3F8;
const COM1_DATA: u16 = COM1_PORT;
const COM1_INT_EN: u16 = COM1_PORT + 1;
const COM1_FIFO_CTRL: u16 = COM1_PORT + 2;
const COM1_LINE_CTRL: u16 = COM1_PORT + 3;
const COM1_MODEM_CTRL: u16 = COM1_PORT + 4;
const COM1_LINE_STATUS: u16 = COM1_PORT + 5;

/// Initialize the serial port (COM1)
pub fn init() {
    unsafe {
        // Disable interrupts
        core::arch::asm!("out dx, al", in("dx") COM1_INT_EN, in("al") 0x00u8);
        
        // Enable DLAB (set baud rate divisor)
        core::arch::asm!("out dx, al", in("dx") COM1_LINE_CTRL, in("al") 0x80u8);
        
        // Set divisor to 3 (lo byte) 38400 baud
        core::arch::asm!("out dx, al", in("dx") COM1_DATA, in("al") 0x03u8);
        
        // Set divisor to 3 (hi byte)
        core::arch::asm!("out dx, al", in("dx") COM1_INT_EN, in("al") 0x00u8);
        
        // 8 bits, no parity, one stop bit
        core::arch::asm!("out dx, al", in("dx") COM1_LINE_CTRL, in("al") 0x03u8);
        
        // Enable FIFO, clear them, with 14-byte threshold
        core::arch::asm!("out dx, al", in("dx") COM1_MODEM_CTRL, in("al") 0x0Bu8);
        
        // Enable FIFO
        core::arch::asm!("out dx, al", in("dx") COM1_FIFO_CTRL, in("al") 0xC7u8);
    }
}

/// Write a single byte to the serial port
fn write_byte(byte: u8) {
    unsafe {
        // Wait for transmit buffer to be empty
        let mut status: u8;
        loop {
            core::arch::asm!("in al, dx", in("dx") COM1_LINE_STATUS, out("al") status);
            if status & 0x20 != 0 {
                break;
            }
        }
        // Write byte to COM1
        core::arch::asm!("out dx, al", in("dx") COM1_DATA, in("al") byte);
    }
}

/// Write a string to the serial port
pub fn write_str(s: &str) {
    for byte in s.bytes() {
        write_byte(byte);
    }
}

/// Print a string to serial port (convenience macro-like function)
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::write_str(core::format_args!($($arg)*).as_str().unwrap_or(""))
    };
}
