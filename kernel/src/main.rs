#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// Import modules
mod graphics;
mod memory;
mod interrupts;
mod drivers;
mod task;
mod console;
mod shell;
mod panic;
mod serial;

use limine::request::{FramebufferRequest, StackSizeRequest};
use limine::framebuffer::Framebuffer;
// use alloc::vec::Vec;
use x86_64;

// Set the base revision to 3 (latest)
#[used]
#[unsafe(link_section = ".limine_requests")]

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

    // Initialize IDT and exception handlers
    interrupts::init();

    // Initialize device drivers
    drivers::init();

    // Initialize task management
    task::init();

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

    // Heap allocation smoke test
    serial::print("Heap test: skipped (no allocator yet)\n");
    // Temporarily disabled until heap allocator is working
    /*
    serial::print("Heap test: allocating Vec...\n");
    let mut nums: Vec<u64> = Vec::new();
    for i in 0..10 {
        nums.push(i * 10);
    }
    serial::print("Vec length: ");
    memory::print_decimal(nums.len() as u64);
    serial::print(", first: ");
    memory::print_decimal(nums[0]);
    serial::print(", last: ");
    memory::print_decimal(*nums.last().unwrap());
    serial::print("\n");
    */

    // Enable interrupts and enter idle loop
    serial::print("Enabling interrupts...\n");
    x86_64::instructions::interrupts::enable();
    
    serial::print("Kernel initialization complete!\n");
    serial::print("Timer ticks every 10ms, keyboard input echoed to serial.\n");
    serial::print("Task scheduler running with preemptive multitasking.\n");
    serial::print("Entering interactive mode...\n");
    
    // Show scheduler stats
    serial::print("Getting scheduler stats...\n");
    let (ready_tasks, switches) = task::SCHEDULER.lock().stats();
    serial::print("Scheduler: ");
    memory::print_decimal(ready_tasks as u64);
    serial::print(" ready tasks, ");
    memory::print_decimal(switches);
    serial::print(" switches\n");
    
    // Initialize console and shell
    serial::print("Checking for framebuffer...\n");
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        serial::print("Framebuffer response received!\n");
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            serial::print("Initializing console...\n");
            
            // Get a static reference to the framebuffer
            // This is safe because the framebuffer is provided by the bootloader
            // and remains valid for the entire kernel lifetime
            let fb_static: &'static Framebuffer<'static> = unsafe { 
                &*(&framebuffer as *const Framebuffer<'static>)
            };
            
            serial::print("Framebuffer: ");
            memory::print_hex(framebuffer.addr() as u64);
            serial::print(" (");
            memory::print_decimal(framebuffer.width() as u64);
            serial::print("x");
            memory::print_decimal(framebuffer.height() as u64);
            serial::print(")\n");
            
            console::init(fb_static);
            shell::init();
            
            serial::print("Console and shell initialized!\n");
        }
    } else {
        serial::print("No framebuffer available - console disabled!\n");
    }
    
    // Optional: Test exception handler (uncomment to test)
    // serial::print("Testing divide by zero exception...\n");
    // let _x = 1 / 0; // This will trigger divide_by_zero_handler
    
    loop {
        x86_64::instructions::hlt();
    }
}

/// Simple console test without heap allocation
fn simple_console_test(framebuffer: &'static Framebuffer<'static>) {
    // Clear screen to black
    let fb_ptr = framebuffer.addr() as *mut u32;
    let width = framebuffer.width() as usize;
    let height = framebuffer.height() as usize;
    let pitch = framebuffer.pitch() as usize / 4;
    
    unsafe {
        // Clear to black
        for y in 0..height {
            for x in 0..width {
                let offset = y * pitch + x;
                *fb_ptr.add(offset) = 0x000000; // Black
            }
        }
        
        // Draw some simple text patterns (without font rendering for now)
        // Draw a white rectangle as a "text area"
        for y in 50..100 {
            for x in 50..600 {
                let offset = y * pitch + x;
                *fb_ptr.add(offset) = 0xFFFFFF; // White
            }
        }
        
        // Draw "POLYGLOT OS" in a simple pattern
        // P
        for y in 60..90 {
            for x in 60..65 {
                let offset = y * pitch + x;
                *fb_ptr.add(offset) = 0x0000FF; // Blue
            }
        }
        for y in 60..65 {
            for x in 60..80 {
                let offset = y * pitch + x;
                *fb_ptr.add(offset) = 0x0000FF; // Blue
            }
        }
        for y in 72..77 {
            for x in 60..80 {
                let offset = y * pitch + x;
                *fb_ptr.add(offset) = 0x0000FF; // Blue
            }
        }
        
        // Add more simple text later...
    }
    
    serial::print("Simple console test completed - you should see text on screen!\n");
}