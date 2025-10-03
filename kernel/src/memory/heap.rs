//! Kernel heap initialization using linked_list_allocator

use linked_list_allocator::LockedHeap;
use x86_64::VirtAddr;

// Choose a kernel heap region in higher-half virtual memory.
// Ensure this does not overlap other mappings. Adjust as needed later.
// Use address below HHDM to avoid conflicts
const HEAP_START: u64 = 0xFFFF_8000_0000_0000; // 512GB - safe area below HHDM
const HEAP_SIZE: u64 = 8 * 1024 * 1024; // 8 MiB for now

// Temporarily disabled until we fix the allocator initialization
// #[global_allocator]
// static GLOBAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the kernel heap: map heap pages and init global allocator
pub fn init() {
    crate::serial::print("Heap: mapping range...\n");
    let start = VirtAddr::new(HEAP_START);

    // Map the heap range
    if let Err(e) = super::paging::try_map_range(start, HEAP_SIZE) {
        crate::serial::print("Heap: failed to map range: ");
        crate::serial::print(e);
        crate::serial::print("\n");
        return;
    }

    crate::serial::print("Heap: initialization skipped for now\n");

    crate::serial::print("Heap area mapped at ");
    crate::memory::print_hex(HEAP_START);
    crate::serial::print(", size ");
    crate::memory::print_size(HEAP_SIZE);
    crate::serial::print("\n");
}
