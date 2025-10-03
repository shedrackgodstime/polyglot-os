//! Task management and scheduling

pub mod task;
pub mod scheduler;
pub mod switch;

pub use task::Task;
pub use scheduler::SCHEDULER;

/// Initialize the task management subsystem
pub fn init() {
    crate::serial::print("Initializing task management...\n");
    
    // Create the initial kernel task
    let kernel_task = Task::new_kernel_task();
    scheduler::SCHEDULER.lock().add_task(kernel_task);
    
    // Create some demo tasks
    create_demo_tasks();
    
    crate::serial::print("Task management initialized.\n");
}

/// Create demonstration tasks to show multitasking
fn create_demo_tasks() {
    // Task 1: Counter task
    let task1 = Task::new(
        task1_main as *const () as u64,
        "Counter Task".into(),
    );
    scheduler::SCHEDULER.lock().add_task(task1);
    
    // Task 2: Letter task  
    let task2 = Task::new(
        task2_main as *const () as u64,
        "Letter Task".into(),
    );
    scheduler::SCHEDULER.lock().add_task(task2);
}

/// Demo task 1: Counts numbers
extern "C" fn task1_main() -> ! {
    let mut counter = 0u64;
    loop {
        crate::serial::print("[Task1: ");
        crate::memory::print_decimal(counter);
        crate::serial::print("] ");
        counter += 1;
        
        // Yield to other tasks
        yield_task();
    }
}

/// Demo task 2: Prints letters
extern "C" fn task2_main() -> ! {
    let letters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut index = 0;
    
    loop {
        crate::serial::print("[Task2: ");
        let letter = [letters[index % letters.len()]];
        let s = core::str::from_utf8(&letter).unwrap_or("?");
        crate::serial::print(s);
        crate::serial::print("] ");
        index += 1;
        
        // Yield to other tasks
        yield_task();
    }
}

/// Yield the current task (cooperative multitasking)
pub fn yield_task() {
    // For now, just a busy wait to simulate work
    for _ in 0..100000 {
        unsafe { core::arch::asm!("nop") };
    }
}
