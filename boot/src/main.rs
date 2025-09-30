#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // Simple "Hello" output in VGA text mode
    unsafe {
        vga_buffer.write_volatile(b'H');
        vga_buffer.add(1).write_volatile(0x0f); // White on black
        vga_buffer.add(2).write_volatile(b'i');
        vga_buffer.add(3).write_volatile(0x0f);
    }

    loop {}
}
