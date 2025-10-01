#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Module declarations
mod boot;
mod serial;
mod framebuffer;

/// Kernel entry point
/// Called by the Limine bootloader after initialization
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize serial port for debug output
    serial::init();
    serial::write_str("Polyglot OS booting...\n");

    // Check if the bootloader supports our base revision
    if !boot::BASE_REVISION.is_supported() {
        serial::write_str("ERROR: Bootloader does not support base revision!\n");
        hcf();
    }

    serial::write_str("Base revision supported!\n");

    // Get framebuffer and draw graphics
    if let Some(framebuffer_response) = boot::FRAMEBUFFER_REQUEST.get_response() {
        serial::write_str("Framebuffer response received!\n");
        framebuffer::draw(framebuffer_response);
    } else {
        serial::write_str("No framebuffer available!\n");
    }

    serial::write_str("Kernel initialized successfully!\n");
    serial::write_str("Halting CPU...\n");

    hcf();
}

/// Halt and catch fire - infinite loop with CPU halt instruction
fn hcf() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler - called when the kernel panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial::write_str("KERNEL PANIC!\n");
    hcf()
}
