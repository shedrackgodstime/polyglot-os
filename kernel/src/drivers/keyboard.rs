//! PS/2 Keyboard driver

use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

/// Keyboard scancode to ASCII mapping (US layout, basic)
const SCANCODE_TO_ASCII: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8, // 0-14
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n', // 15-28
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, // 29-42
    b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, b'*', 0, b' ', // 43-57
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 58-73 (function keys, etc.)
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 74-89
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 90-105
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 // 106-127
];

/// Shifted characters (when Shift is held)
const SHIFTED_CHARS: [u8; 128] = [
    0, 27, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 8, // 0-14
    b'\t', b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', b'\n', // 15-28
    0, b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~', 0, // 29-42
    b'|', b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b'<', b'>', b'?', 0, b'*', 0, b' ', // 43-57
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 58-73
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 74-89
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 90-105
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 // 106-127
];

// Keyboard state
static SHIFT_PRESSED: Mutex<bool> = Mutex::new(false);
static CTRL_PRESSED: Mutex<bool> = Mutex::new(false);

/// Initialize keyboard driver
pub fn init() {
    crate::serial::print("Initializing PS/2 keyboard...\n");
    
    // Enable keyboard interrupt (IRQ 1 -> IRQ 33)
    enable_keyboard_interrupt();
    
    crate::serial::print("Keyboard initialized. Try typing!\n");
}

/// Enable keyboard interrupt in PIC
fn enable_keyboard_interrupt() {
    unsafe {
        let mut pic1_data = Port::new(0x21);
        let mask: u8 = pic1_data.read();
        // Clear bit 1 to enable IRQ 1 (keyboard)
        pic1_data.write(mask & !0x02);
    }
}

/// Keyboard interrupt handler (IRQ 1 -> IRQ 33)
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        let mut port = Port::new(0x60);
        let scancode: u8 = port.read();
        
        handle_scancode(scancode);
        
        // Send EOI to PIC
        crate::interrupts::pic::send_eoi(1);
    }
}

/// Handle a keyboard scancode
fn handle_scancode(scancode: u8) {
    match scancode {
        // Key press events (bit 7 clear)
        0x2A | 0x36 => { // Left/Right Shift pressed
            *SHIFT_PRESSED.lock() = true;
        }
        0x1D => { // Ctrl pressed
            *CTRL_PRESSED.lock() = true;
        }
        
        // Key release events (bit 7 set)
        0xAA | 0xB6 => { // Left/Right Shift released
            *SHIFT_PRESSED.lock() = false;
        }
        0x9D => { // Ctrl released
            *CTRL_PRESSED.lock() = false;
        }
        
        // Regular key press
        key if key < 128 => {
            let shift = *SHIFT_PRESSED.lock();
            let ctrl = *CTRL_PRESSED.lock();
            
            let ascii = if shift {
                SHIFTED_CHARS[key as usize]
            } else {
                SCANCODE_TO_ASCII[key as usize]
            };
            
            if ascii != 0 {
                if ctrl && ascii == b'c' {
                    // Ctrl+C - could be used for interrupt/break
                    crate::serial::print("\n^C\n");
                    crate::shell::handle_keyboard_input(b'\n'); // Send newline to shell
                } else {
                    // Send character to shell
                    crate::shell::handle_keyboard_input(ascii);
                    // Also echo to serial for debugging
                    print_char_serial(ascii);
                }
            }
        }
        
        _ => {
            // Unknown scancode - could log for debugging
        }
    }
}

/// Print a character to serial output (for debugging)
fn print_char_serial(c: u8) {
    if c == b'\n' {
        crate::serial::print("\n");
    } else if c == 8 { // Backspace
        crate::serial::print("\x08 \x08"); // Backspace, space, backspace
    } else if c >= 32 && c <= 126 { // Printable ASCII
        let byte_array = [c];
        let s = core::str::from_utf8(&byte_array).unwrap_or("?");
        crate::serial::print(s);
    }
}
