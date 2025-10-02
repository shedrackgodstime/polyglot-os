#![no_std]
#![no_main]

// Import modules
mod graphics;
mod panic;
mod serial;

use limine::request::{FramebufferRequest, StackSizeRequest};

// Set the base revision to 3 (latest)
#[used]
#[link_section = ".limine_requests"]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

// Request a framebuffer
#[used]
#[link_section = ".limine_requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

// Request a larger stack
#[used]
#[link_section = ".limine_requests"]
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(0x100000);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize serial port for logging
    serial::init();
    serial::print("Polyglot OS booting...\n");

    // Ensure the bootloader supports our base revision
    if !BASE_REVISION.is_supported() {
        panic::hcf();
    }

    serial::print("Base revision supported!\n");

    // Draw to framebuffer if available
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        serial::print("Framebuffer response received!\n");
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            serial::print("Drawing to framebuffer...\n");
            graphics::draw(framebuffer);
            serial::print("Framebuffer drawing complete!\n");
        }
    } else {
        serial::print("No framebuffer available!\n");
    }

    serial::print("Kernel initialized successfully! Halting.\n");

    // Halt the system
    panic::hcf();
}