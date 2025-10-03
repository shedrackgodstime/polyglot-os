//! Context switching implementation

use super::task::{Task, RegisterState};

/// Perform a context switch from old_task to new_task
/// 
/// This function saves the current CPU state to old_task.registers
/// and loads the CPU state from new_task.registers
pub unsafe fn context_switch(old_task: &mut Task, new_task: &mut Task) {
    // Save current task's registers
    unsafe { save_context(&mut old_task.registers); }
    
    // Load new task's registers  
    unsafe { load_context(&new_task.registers); }
}

/// Save current CPU context to the given RegisterState
/// 
/// This is a placeholder - real implementation would use inline assembly
/// to save all CPU registers
unsafe fn save_context(registers: &mut RegisterState) {
    // In a real implementation, this would use inline assembly like:
    // asm!(
    //     "mov {}, rax",
    //     "mov {}, rbx", 
    //     // ... save all registers
    //     out(reg) registers.rax,
    //     out(reg) registers.rbx,
    // );
    
    // For now, we'll simulate saving some state
    registers.rip = get_instruction_pointer();
    registers.rsp = get_stack_pointer();
    registers.rflags = get_flags_register();
}

/// Load CPU context from the given RegisterState
/// 
/// This is a placeholder - real implementation would use inline assembly
/// to restore all CPU registers and jump to the new task
unsafe fn load_context(registers: &RegisterState) {
    // In a real implementation, this would use inline assembly like:
    // asm!(
    //     "mov rax, {}",
    //     "mov rbx, {}",
    //     // ... restore all registers
    //     "jmp {}", // Jump to new RIP
    //     in(reg) registers.rax,
    //     in(reg) registers.rbx,
    //     in(reg) registers.rip,
    // );
    
    // For now, we'll simulate loading some state
    unsafe { set_stack_pointer(registers.rsp); }
    // Note: We can't actually jump to RIP in safe Rust
    // This would require inline assembly in a real implementation
}

/// Get current instruction pointer (simplified)
fn get_instruction_pointer() -> u64 {
    // This is a placeholder - real implementation would capture RIP
    0
}

/// Get current stack pointer
fn get_stack_pointer() -> u64 {
    let rsp: u64;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) rsp);
    }
    rsp
}

/// Set stack pointer
unsafe fn set_stack_pointer(rsp: u64) {
    unsafe { core::arch::asm!("mov rsp, {}", in(reg) rsp); }
}

/// Get flags register
fn get_flags_register() -> u64 {
    let rflags: u64;
    unsafe {
        core::arch::asm!("pushfq; pop {}", out(reg) rflags);
    }
    rflags
}

/// Yield to the scheduler (cooperative multitasking)
pub fn yield_now() {
    // This would trigger a context switch to the next ready task
    // For now, it's just a placeholder
    crate::serial::print("[yield] ");
}
