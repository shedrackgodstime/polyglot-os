//! Programmable Interrupt Controller (PIC) setup

use x86_64::instructions::port::Port;
use spin::Mutex;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;

static PIC1_CMD: Mutex<Port<u8>> = Mutex::new(Port::new(PIC1_COMMAND));
static PIC1_DAT: Mutex<Port<u8>> = Mutex::new(Port::new(PIC1_DATA));
static PIC2_CMD: Mutex<Port<u8>> = Mutex::new(Port::new(PIC2_COMMAND));
static PIC2_DAT: Mutex<Port<u8>> = Mutex::new(Port::new(PIC2_DATA));

/// Remap PIC to avoid conflicts with CPU exceptions
pub fn init() {
    unsafe {
        // Save masks (unused for now)
        let _mask1 = PIC1_DAT.lock().read();
        let _mask2 = PIC2_DAT.lock().read();

        // Start initialization sequence
        PIC1_CMD.lock().write(ICW1_INIT);
        io_wait();
        PIC2_CMD.lock().write(ICW1_INIT);
        io_wait();

        // Set vector offsets (remap to 0x20-0x2F)
        PIC1_DAT.lock().write(0x20); // PIC1 starts at 0x20 (32)
        io_wait();
        PIC2_DAT.lock().write(0x28); // PIC2 starts at 0x28 (40)
        io_wait();

        // Tell PIC1 there's a slave PIC at IRQ2
        PIC1_DAT.lock().write(4);
        io_wait();
        // Tell PIC2 its cascade identity
        PIC2_DAT.lock().write(2);
        io_wait();

        // Set mode
        PIC1_DAT.lock().write(ICW4_8086);
        io_wait();
        PIC2_DAT.lock().write(ICW4_8086);
        io_wait();

        // Mask all interrupts for now
        PIC1_DAT.lock().write(0xFF);
        PIC2_DAT.lock().write(0xFF);
    }
}

/// Send End of Interrupt signal
#[allow(dead_code)]
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            PIC2_CMD.lock().write(0x20);
        }
        PIC1_CMD.lock().write(0x20);
    }
}

/// Small delay for PIC operations
fn io_wait() {
    unsafe {
        Port::new(0x80).write(0u8);
    }
}
