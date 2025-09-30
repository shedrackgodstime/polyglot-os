#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

/// Provide memcpy implementation for compiler intrinsics
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
        i += 1;
    }
    dest
}

/// Provide memset implementation for compiler intrinsics
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *s.add(i) = c as u8;
        }
        i += 1;
    }
    s
}

/// Provide memcmp implementation for compiler intrinsics
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        unsafe {
            let a = *s1.add(i);
            let b = *s2.add(i);
            if a != b {
                return a as i32 - b as i32;
            }
        }
        i += 1;
    }
    0
}

/// Clear the VGA screen
unsafe fn clear_screen() {
    for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
        *VGA_BUFFER.add(i * 2) = b' ';
        *VGA_BUFFER.add(i * 2 + 1) = 0x0f; // White on black
    }
}

/// Write a string to VGA at a specific position
unsafe fn write_at(x: usize, y: usize, text: &[u8], color: u8) {
    let offset = (y * VGA_WIDTH + x) * 2;
    for (i, &byte) in text.iter().enumerate() {
        *VGA_BUFFER.add(offset + i * 2) = byte;
        *VGA_BUFFER.add(offset + i * 2 + 1) = color;
    }
}

/// Draw a box border
unsafe fn draw_box(x: usize, y: usize, width: usize, height: usize, color: u8) {
    // Top and bottom borders
    for i in 0..width {
        write_at(x + i, y, b"=", color);
        write_at(x + i, y + height - 1, b"=", color);
    }
    // Side borders
    for i in 1..height - 1 {
        write_at(x, y + i, b"|", color);
        write_at(x + width - 1, y + i, b"|", color);
    }
}

/// Kernel entry point
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        // Clear screen
        clear_screen();
        
        // Draw a nice box
        draw_box(10, 5, 60, 15, 0x0b); // Cyan
        
        // Title
        write_at(25, 7, b"POLYGLOT OS KERNEL", 0x0e); // Yellow
        write_at(28, 8, b"Edition 2024", 0x07); // Light gray
        
        // Status messages
        write_at(15, 10, b"[OK] Kernel loaded successfully", 0x0a); // Green
        write_at(15, 11, b"[OK] VGA text mode initialized", 0x0a);
        write_at(15, 12, b"[OK] Memory intrinsics active", 0x0a);
        write_at(15, 13, b"[OK] Kernel is running!", 0x0a);
        
        // Footer
        write_at(20, 16, b"Boot stage complete - Kernel active", 0x0f);
        
        // Blinking cursor indicator at bottom
        let mut counter: usize = 0;
        loop {
            // Simple animation - blink a character
            let char = if (counter / 50000000) % 2 == 0 { b'*' } else { b' ' };
            write_at(39, 23, &[char], 0x0c); // Red
            counter = counter.wrapping_add(1);
        }
    }
}

/// Panic handler for the kernel
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_at(0, 24, b"KERNEL PANIC!", 0x4f); // White on red
    }
    loop {}
}

/// Entry point called by bootloader
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel_main()
}
