//! IDT setup and exception handlers

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::registers::control::Cr2;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Divide-by-zero (#DE)
        idt.divide_error.set_handler_fn(divide_by_zero_handler);

        // Page fault (#PF)
        idt.page_fault.set_handler_fn(page_fault_handler);

        // Double fault (#DF) with IST stack
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // Hardware interrupts (remapped by PIC)
        idt[32].set_handler_fn(crate::drivers::timer::timer_interrupt_handler);   // IRQ 0 -> 32
        idt[33].set_handler_fn(crate::drivers::keyboard::keyboard_interrupt_handler); // IRQ 1 -> 33

        idt
    };
}

/// Load the IDT
pub unsafe fn init() {
    IDT.load();
}

extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    crate::serial::print("EXCEPTION: DIVIDE BY ZERO\n");
    log_stack_frame(&stack_frame);
    crate::panic::hcf();
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let addr = Cr2::read().expect("CR2 read failed").as_u64();
    crate::serial::print("EXCEPTION: PAGE FAULT\n");
    crate::serial::print("Accessed Address: ");
    crate::memory::print_hex(addr);
    crate::serial::print("\nError Code: ");
    crate::memory::print_hex(error_code.bits() as u64);
    crate::serial::print("\n");
    log_stack_frame(&stack_frame);
    crate::panic::hcf();
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    crate::serial::print("EXCEPTION: DOUBLE FAULT\n");
    log_stack_frame(&stack_frame);
    crate::panic::hcf();
}

fn log_stack_frame(stack_frame: &InterruptStackFrame) {
    let ip = stack_frame.instruction_pointer.as_u64();
    let sp = stack_frame.stack_pointer.as_u64();
    let cs = stack_frame.code_segment.0 as u64;

    crate::serial::print("  RIP: ");
    crate::memory::print_hex(ip);
    crate::serial::print("  RSP: ");
    crate::memory::print_hex(sp);
    crate::serial::print("  CS: ");
    crate::memory::print_hex(cs);
    crate::serial::print("\n");
}
