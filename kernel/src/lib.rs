#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Kernel entry point
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    unsafe {
        *vga_buffer = b'K';          // Character
        *vga_buffer.add(1) = 0x0f;  // Color (white on black)
    }

    loop {}
}

/// Panic handler for the kernel
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
