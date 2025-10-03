//! Task scheduler implementation

use alloc::collections::VecDeque;
use spin::Mutex;
use lazy_static::lazy_static;

use super::task::{Task, TaskId, TaskState};

/// Global task scheduler
lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

/// Round-robin task scheduler
pub struct Scheduler {
    /// Queue of ready tasks
    ready_queue: VecDeque<Task>,
    /// Currently running task
    current_task: Option<Task>,
    /// Task switch counter
    switch_count: u64,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            current_task: None,
            switch_count: 0,
        }
    }
    
    /// Add a task to the scheduler
    pub fn add_task(&mut self, mut task: Task) {
        let task_id = task.id.0;
        let task_name = task.name.clone();
        
        task.state = TaskState::Ready;
        self.ready_queue.push_back(task);
        
        crate::serial::print("Added task ");
        crate::memory::print_decimal(task_id);
        crate::serial::print(" (");
        crate::serial::print(&task_name);
        crate::serial::print(")\n");
    }
    
    /// Get the currently running task
    pub fn current_task(&self) -> Option<&Task> {
        self.current_task.as_ref()
    }
    
    /// Get the current task ID
    pub fn current_task_id(&self) -> Option<TaskId> {
        self.current_task.as_ref().map(|t| t.id)
    }
    
    /// Schedule the next task (round-robin)
    pub fn schedule(&mut self) -> Option<&mut Task> {
        // Move current task back to ready queue if it's still running
        if let Some(mut current) = self.current_task.take() {
            if current.state == TaskState::Running {
                current.state = TaskState::Ready;
                self.ready_queue.push_back(current);
            }
        }
        
        // Get next ready task
        if let Some(mut next_task) = self.ready_queue.pop_front() {
            next_task.state = TaskState::Running;
            self.current_task = Some(next_task);
            self.switch_count += 1;
            
            if let Some(ref task) = self.current_task {
                if self.switch_count % 100 == 0 { // Log every 100 switches
                    crate::serial::print("Switch #");
                    crate::memory::print_decimal(self.switch_count);
                    crate::serial::print(" -> Task ");
                    crate::memory::print_decimal(task.id.0);
                    crate::serial::print("\n");
                }
            }
        }
        
        self.current_task.as_mut()
    }
    
    /// Get scheduler statistics
    pub fn stats(&self) -> (usize, u64) {
        (self.ready_queue.len(), self.switch_count)
    }
    
    /// Block the current task
    pub fn block_current_task(&mut self) {
        if let Some(ref mut task) = self.current_task {
            task.state = TaskState::Blocked;
        }
    }
    
    /// Terminate the current task
    pub fn terminate_current_task(&mut self) {
        if let Some(ref mut task) = self.current_task {
            task.state = TaskState::Terminated;
        }
    }
}

/// Called by timer interrupt to perform preemptive scheduling
pub fn timer_tick() {
    // For now, just track that we got a timer tick
    // Real context switching will be implemented in switch.rs
    static mut TICK_COUNT: u64 = 0;
    
    unsafe {
        TICK_COUNT += 1;
        
        // Every 10 ticks (100ms), try to schedule
        if TICK_COUNT % 10 == 0 {
            let mut scheduler = SCHEDULER.lock();
            if let Some(_next_task) = scheduler.schedule() {
                // TODO: Perform actual context switch
                // For now, just log the scheduling decision
            }
        }
    }
}
