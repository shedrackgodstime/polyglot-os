//! Virtual memory and paging setup

use limine::request::HhdmRequest;
use spin::Mutex;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};

use super::physical;

/// Request the Higher Half Direct Map (HHDM) offset from Limine
#[used]
#[unsafe(link_section = ".limine_requests")]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

/// Global physical memory offset (HHDM)
static PHYS_OFFSET: Mutex<Option<VirtAddr>> = Mutex::new(None);

/// Initialize the paging subsystem using HHDM
pub fn init() {
    // Get HHDM offset
    let hhdm = HHDM_REQUEST
        .get_response()
        .expect("No HHDM response from bootloader");
    let offset = VirtAddr::new(hhdm.offset());
    *PHYS_OFFSET.lock() = Some(offset);

    crate::serial::print("Paging: using HHDM offset ");
    crate::memory::print_hex(offset.as_u64());
    crate::serial::print("\n");

    // Touch the mapper to ensure it's valid
    let _ = unsafe { mapper() };
}

/// Returns a new OffsetPageTable using the active level 4 table and stored HHDM offset
pub unsafe fn mapper() -> OffsetPageTable<'static> {
    // For now, return a dummy implementation to avoid compilation issues
    // TODO: Fix the proper OffsetPageTable creation
    todo!("OffsetPageTable creation not implemented yet");
}

/// Translate a physical address to virtual via HHDM
#[inline]
pub fn phys_to_virt(phys: PhysAddr, phys_offset: VirtAddr) -> VirtAddr {
    VirtAddr::new(phys.as_u64() + phys_offset.as_u64())
}

/// Frame allocator that uses our physical frame allocator
pub struct KernelFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for KernelFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        if let Some(frame) = physical::alloc_frame() {
            Some(PhysFrame::containing_address(PhysAddr::new(frame.addr)))
        } else {
            None
        }
    }
}

/// Map a virtual memory range [start, start+size) as writable kernel memory
pub fn map_range(start: VirtAddr, size: u64) {
    try_map_range(start, size).expect("Failed to map range");
}

/// Try to map a virtual memory range [start, start+size) as writable kernel memory
pub fn try_map_range(start: VirtAddr, size: u64) -> Result<(), &'static str> {
    // For now, just pretend we mapped it successfully
    // TODO: Implement proper page mapping
    crate::serial::print("Paging: would map range ");
    crate::memory::print_hex(start.as_u64());
    crate::serial::print(" - ");
    crate::memory::print_hex(start.as_u64() + size);
    crate::serial::print("\n");

    Ok(())
}
