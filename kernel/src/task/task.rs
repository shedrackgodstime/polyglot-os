//! Task structure and management

use alloc::string::String;
use x86_64::VirtAddr;
use core::sync::atomic::{AtomicU64, Ordering};

/// Task ID counter
static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(0);

/// Unique task identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskId(pub u64);

/// Task execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// CPU register state for context switching
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RegisterState {
    // General purpose registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    
    // Instruction pointer and flags
    pub rip: u64,
    pub rflags: u64,
}

impl Default for RegisterState {
    fn default() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0,
            rflags: 0x202, // Interrupts enabled, reserved bit set
        }
    }
}

/// Task control block
#[derive(Debug)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub state: TaskState,
    pub registers: RegisterState,
    pub stack_base: VirtAddr,
    pub stack_size: usize,
}

impl Task {
    /// Stack size for new tasks (64 KB)
    const STACK_SIZE: usize = 64 * 1024;
    
    /// Generate next task ID
    fn next_id() -> TaskId {
        let id = NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst);
        TaskId(id)
    }
    
    /// Create a new task with given entry point
    pub fn new(entry_point: u64, name: String) -> Self {
        let id = Self::next_id();
        
        // Allocate stack for the task
        let stack_base = Self::allocate_stack();
        let stack_top = stack_base + Self::STACK_SIZE as u64;
        
        let mut registers = RegisterState::default();
        registers.rip = entry_point;
        registers.rsp = (stack_top - 8).as_u64(); // Leave space for return address
        registers.rbp = stack_top.as_u64();
        
        Self {
            id,
            name,
            state: TaskState::Ready,
            registers,
            stack_base,
            stack_size: Self::STACK_SIZE,
        }
    }
    
    /// Create the initial kernel task (current execution context)
    pub fn new_kernel_task() -> Self {
        let id = Self::next_id();
        
        Self {
            id,
            name: "Kernel".into(),
            state: TaskState::Running,
            registers: RegisterState::default(), // Will be filled during first context switch
            stack_base: VirtAddr::new(0), // Kernel uses current stack
            stack_size: 0,
        }
    }
    
    /// Allocate stack memory for a task
    fn allocate_stack() -> VirtAddr {
        use crate::memory::physical;
        
        // Allocate physical frames for the stack
        let frames_needed = (Self::STACK_SIZE + 4095) / 4096; // Round up to pages
        let mut stack_frames = alloc::vec::Vec::new();
        
        for _ in 0..frames_needed {
            if let Some(frame) = physical::alloc_frame() {
                stack_frames.push(frame);
            } else {
                panic!("Out of memory allocating task stack");
            }
        }
        
        // For now, use identity mapping (this is simplified)
        // In a real OS, we'd map these to virtual addresses
        let stack_base = VirtAddr::new(stack_frames[0].addr);
        
        // TODO: Properly map stack pages to virtual memory
        // For now, we assume identity mapping works
        
        stack_base
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        // TODO: Free allocated stack memory
        crate::serial::print("Task ");
        crate::memory::print_decimal(self.id.0);
        crate::serial::print(" dropped\n");
    }
}
