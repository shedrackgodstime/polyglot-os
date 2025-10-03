//! Memory management subsystem
//!
//! This module provides physical and virtual memory management for the kernel.

pub mod physical;
pub mod paging;
pub mod heap;

use limine::memory_map::EntryType;
use limine::request::MemoryMapRequest;

/// Request memory map from Limine bootloader
#[used]
#[unsafe(link_section = ".limine_requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

/// Initialize the memory management subsystem
pub fn init() {
    crate::serial::print("Initializing memory management...\n");

    // Get memory map from Limine
    let memory_map_response = MEMORY_MAP_REQUEST
        .get_response()
        .expect("Failed to get memory map from bootloader");

    let entries = memory_map_response.entries();
    
    crate::serial::print("Memory map:\n");
    
    let mut total_memory: u64 = 0;
    let mut usable_memory: u64 = 0;
    
    for entry in entries.iter() {
        let entry_type_str = match entry.entry_type {
            EntryType::USABLE => {
                usable_memory += entry.length;
                "USABLE"
            }
            EntryType::RESERVED => "RESERVED",
            EntryType::ACPI_RECLAIMABLE => "ACPI_RECLAIMABLE",
            EntryType::ACPI_NVS => "ACPI_NVS",
            EntryType::BAD_MEMORY => "BAD_MEMORY",
            EntryType::BOOTLOADER_RECLAIMABLE => {
                usable_memory += entry.length;
                "BOOTLOADER_RECLAIMABLE"
            }
            EntryType::EXECUTABLE_AND_MODULES => "EXECUTABLE_AND_MODULES",
            EntryType::FRAMEBUFFER => "FRAMEBUFFER",
            _ => "UNKNOWN",
        };
        
        total_memory += entry.length;
        
        // Print memory region
        let base = entry.base;
        let length = entry.length;
        let end = base + length;
        
        crate::serial::print("  ");
        print_hex(base);
        crate::serial::print(" - ");
        print_hex(end);
        crate::serial::print(" (");
        print_size(length);
        crate::serial::print(") ");
        crate::serial::print(entry_type_str);
        crate::serial::print("\n");
    }
    
    crate::serial::print("\nTotal memory: ");
    print_size(total_memory);
    crate::serial::print("\nUsable memory: ");
    print_size(usable_memory);
    crate::serial::print("\n");
    
    // Initialize physical frame allocator
    physical::init(entries);
    // Initialize paging using HHDM
    crate::serial::print("Initializing paging...\n");
    paging::init();
    crate::serial::print("Paging initialized.\n");
    // Initialize kernel heap
    crate::serial::print("Initializing heap...\n");
    heap::init();
    crate::serial::print("Heap initialized.\n");
    
    crate::serial::print("Memory management initialized!\n");
}

/// Print a hexadecimal number
pub fn print_hex(value: u64) {
    crate::serial::print("0x");
    let hex_chars = b"0123456789ABCDEF";
    let mut buffer = [0u8; 16];
    let mut i = 0;
    let mut v = value;
    
    if v == 0 {
        crate::serial::print("0");
        return;
    }
    
    while v > 0 {
        buffer[i] = hex_chars[(v & 0xF) as usize];
        v >>= 4;
        i += 1;
    }
    
    // Print in reverse order
    while i > 0 {
        i -= 1;
        crate::serial::print(core::str::from_utf8(&[buffer[i]]).unwrap());
    }
}

/// Print a size in human-readable format
pub fn print_size(bytes: u64) {
    if bytes < 1024 {
        print_decimal(bytes);
        crate::serial::print(" B");
    } else if bytes < 1024 * 1024 {
        print_decimal(bytes / 1024);
        crate::serial::print(" KB");
    } else if bytes < 1024 * 1024 * 1024 {
        print_decimal(bytes / (1024 * 1024));
        crate::serial::print(" MB");
    } else {
        print_decimal(bytes / (1024 * 1024 * 1024));
        crate::serial::print(" GB");
    }
}

/// Print a decimal number
pub fn print_decimal(value: u64) {
    if value == 0 {
        crate::serial::print("0");
        return;
    }
    
    let mut buffer = [0u8; 20];
    let mut i = 0;
    let mut v = value;
    
    while v > 0 {
        buffer[i] = b'0' + (v % 10) as u8;
        v /= 10;
        i += 1;
    }
    
    // Print in reverse order
    while i > 0 {
        i -= 1;
        crate::serial::print(core::str::from_utf8(&[buffer[i]]).unwrap());
    }
}
