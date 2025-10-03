//! Simple physical memory frame allocator
//!
//! This module provides a simple bump allocator for physical memory frames.
//! It's much simpler than a bitmap allocator and avoids the hanging issue.

use limine::memory_map::Entry;
use spin::Mutex;

/// Size of a physical frame (4KB)
pub const FRAME_SIZE: usize = 4096;

/// Simple bump allocator - starts at 2MB to avoid low memory
static NEXT_FRAME: Mutex<u64> = Mutex::new(0x200000);

/// Maximum memory we'll allocate (limit to 64MB for safety)
const MAX_FRAME: u64 = 0x200000 + (64 * 1024 * 1024);

/// Represents a physical memory frame
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysFrame {
    pub addr: u64,
}

impl PhysFrame {
    /// Create a new PhysFrame from an address
    pub fn containing_address(addr: u64) -> Self {
        Self {
            addr: addr & !0xFFF, // Align to 4KB boundary
        }
    }
}

/// Initialize the physical frame allocator (simple version)
pub fn init(_entries: &[&Entry]) {
    // For the simple allocator, we don't need to parse the memory map
    // We just start allocating from 2MB onwards
    crate::serial::print("Simple physical allocator initialized (bump allocator)\n");
    crate::serial::print("Allocation range: 2MB - 66MB\n");
}

/// Allocate a physical frame
pub fn alloc_frame() -> Option<PhysFrame> {
    let mut next = NEXT_FRAME.lock();
    
    if *next >= MAX_FRAME {
        return None; // Out of memory
    }
    
    let frame = PhysFrame::containing_address(*next);
    *next += FRAME_SIZE as u64;
    
    Some(frame)
}

/// Deallocate a physical frame (no-op for bump allocator)
pub fn dealloc_frame(_frame: PhysFrame) {
    // Bump allocators don't support deallocation
    // In a real OS, you'd want a proper allocator
}

/// Get the amount of free memory (estimate)
pub fn free_memory() -> u64 {
    let next = *NEXT_FRAME.lock();
    if next < MAX_FRAME {
        MAX_FRAME - next
    } else {
        0
    }
}

/// Get the number of free frames (estimate)
pub fn free_frames() -> usize {
    (free_memory() / FRAME_SIZE as u64) as usize
}
