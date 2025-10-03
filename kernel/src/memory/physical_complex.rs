//! Physical memory frame allocator
//!
//! This module manages physical memory frames (4KB pages).
//! It uses a bitmap allocator to track which frames are free/allocated.

use limine::memory_map::{Entry, EntryType};
use spin::Mutex;

/// Size of a physical frame (4KB)
pub const FRAME_SIZE: usize = 4096;

/// Physical frame allocator
static FRAME_ALLOCATOR: Mutex<Option<BitmapAllocator>> = Mutex::new(None);

/// Represents a physical memory frame
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysFrame {
    pub addr: u64,
}

impl PhysFrame {
    /// Create a new physical frame from an address
    pub fn from_addr(addr: u64) -> Self {
        assert!(addr % FRAME_SIZE as u64 == 0, "Address must be frame-aligned");
        Self { addr }
    }

    /// Get the frame number
    pub fn number(&self) -> u64 {
        self.addr / FRAME_SIZE as u64
    }
}

/// Bitmap-based physical frame allocator
struct BitmapAllocator {
    /// Bitmap storage (each bit represents one frame)
    bitmap: &'static mut [u64],
    /// Total number of frames
    total_frames: usize,
    /// Number of free frames
    free_frames: usize,
    /// Base address of the bitmap
    #[allow(dead_code)]
    bitmap_base: u64,
}

impl BitmapAllocator {
    /// Create a new bitmap allocator
    fn new(entries: &[&Entry]) -> Self {
        // Find the largest usable memory region for our bitmap
        // Limit to reasonable memory range (ignore very high addresses like framebuffer)
        let mut max_addr = 0u64;
        for entry in entries {
            if matches!(entry.entry_type, EntryType::USABLE | EntryType::BOOTLOADER_RECLAIMABLE) {
                let end_addr = entry.base + entry.length;
                // Only consider memory below 4GB to avoid framebuffer and other high mappings
                if end_addr < 0x100000000 {
                    max_addr = max_addr.max(end_addr);
                }
        }

        let total_frames = (max_addr / FRAME_SIZE as u64) as usize;
        let bitmap_size = (total_frames + 63) / 64; // Round up to u64 chunks

        crate::serial::print("Physical allocator: ");
        crate::serial::print_decimal(total_frames as u64);
        crate::serial::print(" frames (");
        crate::memory::print_size(total_frames as u64 * FRAME_SIZE as u64);
        crate::serial::print(")\n");

        // Find a suitable location for the bitmap in usable memory
        let mut bitmap_base: u64 = 0;

        for entry in entries.iter() {
            if entry.entry_type == EntryType::USABLE && entry.length >= bitmap_bytes as u64 {
                bitmap_base = entry.base;
                break;
            }
        }

        if bitmap_base == 0 {
            panic!("No suitable memory region for bitmap allocator");
        }

        crate::serial::print("Bitmap location: ");
        crate::memory::print_hex(bitmap_base);
        crate::serial::print(" (");
        crate::memory::print_size(bitmap_bytes as u64);
        crate::serial::print(")\n");

        // Create bitmap slice
        let bitmap = unsafe {
            core::slice::from_raw_parts_mut(bitmap_base as *mut u64, bitmap_size)
        };

        // Initialize bitmap - mark all frames as allocated
        for chunk in bitmap.iter_mut() {
            *chunk = u64::MAX;
        }

        let mut allocator = Self {
            bitmap,
            total_frames,
            free_frames: 0,
            bitmap_base,
        };

        // Mark usable regions as free
        for entry in entries.iter() {
            if entry.entry_type == EntryType::USABLE 
                || entry.entry_type == EntryType::BOOTLOADER_RECLAIMABLE {
                allocator.mark_region_free(entry.base, entry.length);
            }
        }

        // Mark the bitmap itself as allocated
        allocator.mark_region_allocated(bitmap_base, bitmap_bytes as u64);

        crate::serial::print("Free frames: ");
        crate::memory::print_decimal(allocator.free_frames as u64);
        crate::serial::print(" (");
        crate::memory::print_size(allocator.free_frames as u64 * FRAME_SIZE as u64);
        crate::serial::print(")\n");

        allocator
    }

    /// Mark a memory region as free
    fn mark_region_free(&mut self, base: u64, length: u64) {
        let start_frame = base / FRAME_SIZE as u64;
        let end_frame = (base + length) / FRAME_SIZE as u64;

        for frame in start_frame..end_frame {
            if (frame as usize) < self.total_frames {
                self.mark_frame_free(frame);
            }
        }
    }

    /// Mark a memory region as allocated
    fn mark_region_allocated(&mut self, base: u64, length: u64) {
        let start_frame = base / FRAME_SIZE as u64;
        let end_frame = (base + length + FRAME_SIZE as u64 - 1) / FRAME_SIZE as u64;

        for frame in start_frame..end_frame {
            if (frame as usize) < self.total_frames {
                self.mark_frame_allocated(frame);
            }
        }
    }

    /// Mark a frame as free
    fn mark_frame_free(&mut self, frame: u64) {
        let chunk_index = (frame / 64) as usize;
        let bit_index = frame % 64;

        if chunk_index >= self.bitmap.len() {
            return;
        }

        let was_allocated = (self.bitmap[chunk_index] & (1 << bit_index)) != 0;
        self.bitmap[chunk_index] &= !(1 << bit_index);

        if was_allocated {
            self.free_frames += 1;
        }
    }

    /// Mark a frame as allocated
    fn mark_frame_allocated(&mut self, frame: u64) {
        let chunk_index = (frame / 64) as usize;
        let bit_index = frame % 64;

        if chunk_index >= self.bitmap.len() {
            return;
        }

        let was_free = (self.bitmap[chunk_index] & (1 << bit_index)) == 0;
        self.bitmap[chunk_index] |= 1 << bit_index;

        if was_free && self.free_frames > 0 {
            self.free_frames -= 1;
        }
    }

    /// Allocate a physical frame
    fn allocate(&mut self) -> Option<PhysFrame> {
        if self.free_frames == 0 {
            return None;
        }

        // Find first free frame
        for (chunk_index, chunk) in self.bitmap.iter_mut().enumerate() {
            if *chunk != u64::MAX {
                // Found a chunk with free frames
                for bit_index in 0..64 {
                    if (*chunk & (1 << bit_index)) == 0 {
                        // Found a free frame
                        *chunk |= 1 << bit_index;
                        self.free_frames -= 1;

                        let frame_number = (chunk_index * 64 + bit_index) as u64;
                        let addr = frame_number * FRAME_SIZE as u64;
                        return Some(PhysFrame::from_addr(addr));
                    }
                }
            }
        }

        None
    }

    /// Deallocate a physical frame
    fn deallocate(&mut self, frame: PhysFrame) {
        self.mark_frame_free(frame.number());
    }

    /// Get the number of free frames
    fn free_frames(&self) -> usize {
        self.free_frames
    }
}

/// Initialize the physical frame allocator
pub fn init(entries: &[&Entry]) {
    let allocator = BitmapAllocator::new(entries);
    *FRAME_ALLOCATOR.lock() = Some(allocator);
}

/// Allocate a physical frame
pub fn alloc_frame() -> Option<PhysFrame> {
    FRAME_ALLOCATOR.lock().as_mut()?.allocate()
}

/// Deallocate a physical frame
pub fn dealloc_frame(frame: PhysFrame) {
    if let Some(allocator) = FRAME_ALLOCATOR.lock().as_mut() {
        allocator.deallocate(frame);
    }
}

/// Get the number of free frames
pub fn free_frames() -> usize {
    FRAME_ALLOCATOR.lock().as_ref().map_or(0, |a| a.free_frames())
}

/// Get the total amount of free memory in bytes
pub fn free_memory() -> u64 {
    free_frames() as u64 * FRAME_SIZE as u64
}
