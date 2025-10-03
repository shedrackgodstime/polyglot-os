//! Interrupts subsystem: IDT, GDT, PIC, and exception handlers

pub mod gdt;
pub mod idt;
pub mod pic;

/// Initialize the interrupts subsystem (GDT, IDT, PIC)
pub fn init() {
    crate::serial::print("Initializing GDT...\n");
    gdt::init();
    
    crate::serial::print("Initializing IDT and exception handlers...\n");
    unsafe { idt::init(); }
    
    crate::serial::print("Remapping PIC...\n");
    pic::init();
    
    crate::serial::print("Interrupts subsystem initialized.\n");
}
