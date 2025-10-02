#![no_std]
#![no_main]

// Import modules
mod graphics;
mod memory;
mod panic;
mod serial;

use limine::request::{FramebufferRequest, StackSizeRequest};

// Set the base revision to 3 (latest)
#[used]
#[unsafe(link_section = ".limine_requests")]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

// Request a framebuffer
#[used]
#[unsafe(link_section = ".limine_requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

// Request a larger stack
#[used]
#[unsafe(link_section = ".limine_requests")]
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(0x100000);

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Initialize serial port for logging
    serial::init();
    serial::print("Polyglot OS booting...\n");

    // Ensure the bootloader supports our base revision
    if !BASE_REVISION.is_supported() {
        panic::hcf();
    }

    serial::print("Base revision supported!\n");

    // Initialize memory management
    memory::init();

    // Test frame allocation
    serial::print("\nTesting frame allocation...\n");
    if let Some(frame1) = memory::physical::alloc_frame() {
        serial::print("Allocated frame at: ");
        memory::print_hex(frame1.addr);
        serial::print("\n");
        
        if let Some(frame2) = memory::physical::alloc_frame() {
            serial::print("Allocated frame at: ");
            memory::print_hex(frame2.addr);
            serial::print("\n");
            
            // Free the frames
            memory::physical::dealloc_frame(frame1);
            memory::physical::dealloc_frame(frame2);
            serial::print("Freed both frames\n");
        }
    }
    
    serial::print("Free memory: ");
    memory::print_size(memory::physical::free_memory());
    serial::print("\n\n");

    // Draw to framebuffer if available
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        serial::print("Framebuffer response received!\n");
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            serial::print("Drawing to framebuffer...\n");
            graphics::draw(&framebuffer);
            serial::print("Framebuffer drawing complete!\n");
        }
    } else {
        serial::print("No framebuffer available!\n");
    }

    serial::print("Kernel initialized successfully! Halting.\n");

    // Halt the system
    panic::hcf();
}