# Polyglot OS - Implementation Plan (Option A)

**Goal**: Build solid kernel foundations before integrating multi-language runtimes

**Timeline**: ~8-12 weeks for core kernel, then polyglot features

---

## Phase 1: Memory Management (Week 1-2)

### 1.1 Physical Memory Allocator
**What**: Track and allocate physical memory pages (4KB chunks)

**Implementation**:
- Parse Limine memory map to find usable RAM regions
- Implement bitmap or stack-based physical frame allocator
- Track allocated/free pages
- Provide `alloc_frame()` and `free_frame()` functions

**Files to create**:
- `kernel/src/memory/mod.rs` - Memory subsystem entry point
- `kernel/src/memory/physical.rs` - Physical frame allocator
- `kernel/src/memory/limine_parser.rs` - Parse Limine memory map

**Dependencies**:
- Limine `MemoryMapRequest` (already have limine crate)

**Success criteria**:
- Can allocate and free physical frames
- Serial output shows available memory
- No memory leaks in allocation/deallocation

---

### 1.2 Virtual Memory & Paging
**What**: Set up page tables for virtual memory management

**Implementation**:
- Create page table structures (PML4, PDPT, PD, PT)
- Identity map kernel code/data
- Map framebuffer to virtual address
- Implement page fault handler
- Enable paging (should already be enabled by Limine, but verify)

**Files to create**:
- `kernel/src/memory/paging.rs` - Page table management
- `kernel/src/memory/address.rs` - Virtual/Physical address types

**Dependencies**:
- Physical allocator from 1.1
- `x86_64` crate (optional, for CR3 manipulation)

**Success criteria**:
- Kernel runs with custom page tables
- Can map/unmap virtual addresses
- Page fault handler catches invalid accesses

---

### 1.3 Heap Allocator
**What**: Enable Rust's `alloc` crate for dynamic memory

**Implementation**:
- Implement `GlobalAlloc` trait
- Use existing allocator crate: `linked_list_allocator` or `buddy_system_allocator`
- Allocate heap region from physical memory
- Map heap to virtual address space

**Files to modify**:
- `kernel/Cargo.toml` - Add `alloc` to build-std, add allocator crate
- `kernel/src/memory/heap.rs` - Heap allocator setup

**Dependencies**:
- Paging from 1.2
- Physical allocator from 1.1

**Success criteria**:
- Can use `Vec`, `Box`, `String` in kernel
- No panics on allocation
- Can allocate/deallocate dynamically

---

## Phase 2: Interrupt Handling (Week 3)

### 2.1 IDT Setup
**What**: Create Interrupt Descriptor Table for exception/interrupt handling

**Implementation**:
- Define IDT structure (256 entries)
- Create interrupt handler stubs (assembly + Rust)
- Load IDT using `lidt` instruction
- Handle CPU exceptions (divide by zero, page fault, double fault, etc.)

**Files to create**:
- `kernel/src/interrupts/mod.rs` - Interrupt subsystem
- `kernel/src/interrupts/idt.rs` - IDT structure and loading
- `kernel/src/interrupts/handlers.rs` - Exception handlers
- `kernel/src/interrupts/asm.s` - Assembly interrupt stubs (optional)

**Dependencies**:
- Consider using `x86_64` crate for IDT structures

**Success criteria**:
- IDT loaded successfully
- Divide by zero triggers handler (test with `1/0`)
- Page fault handler prints fault address
- Double fault handler prevents triple fault

---

### 2.2 PIC/APIC Setup
**What**: Configure interrupt controller for hardware interrupts

**Implementation**:
- Remap PIC (8259) to avoid conflicts with CPU exceptions
- Or use APIC if available (check ACPI tables)
- Enable/disable specific IRQ lines
- Send EOI (End of Interrupt) signals

**Files to create**:
- `kernel/src/interrupts/pic.rs` - PIC driver
- `kernel/src/interrupts/apic.rs` - APIC driver (optional, later)

**Success criteria**:
- PIC remapped to IRQ 32-47
- Can enable/disable IRQs
- EOI signals sent correctly

---

## Phase 3: Device Drivers (Week 4)

### 3.1 Timer Driver
**What**: Programmable Interval Timer (PIT) or APIC timer

**Implementation**:
- Configure PIT to fire at ~100Hz (10ms intervals)
- Register timer interrupt handler (IRQ 0)
- Implement tick counter
- Basic sleep/delay functions

**Files to create**:
- `kernel/src/drivers/timer.rs` - Timer driver

**Success criteria**:
- Timer interrupt fires regularly
- Tick counter increments
- Can implement `sleep_ms(100)` function

---

### 3.2 Keyboard Driver
**What**: PS/2 keyboard input

**Implementation**:
- Register keyboard interrupt handler (IRQ 1)
- Read scancodes from port 0x60
- Implement scancode-to-ASCII translation
- Buffer keyboard input

**Files to create**:
- `kernel/src/drivers/keyboard.rs` - Keyboard driver
- `kernel/src/drivers/scancode.rs` - Scancode tables

**Success criteria**:
- Can read keyboard input
- Serial output shows pressed keys
- Handles special keys (Shift, Ctrl, etc.)

---

## Phase 4: Process Management (Week 5-6)

### 4.1 Task Structure
**What**: Define process/task data structure

**Implementation**:
- Create `Task` struct with:
  - Task ID
  - Register state (saved context)
  - Page table pointer
  - Stack pointer
  - State (running, ready, blocked)
- Task list/queue management

**Files to create**:
- `kernel/src/task/mod.rs` - Task management
- `kernel/src/task/task.rs` - Task structure
- `kernel/src/task/scheduler.rs` - Task scheduler

**Success criteria**:
- Can create task structures
- Task list management works

---

### 4.2 Context Switching
**What**: Switch between tasks

**Implementation**:
- Save/restore CPU registers
- Switch page tables (CR3)
- Switch stack pointers
- Assembly routine for context switch

**Files to create**:
- `kernel/src/task/switch.rs` - Context switching logic
- `kernel/src/task/switch.s` - Assembly context switch

**Success criteria**:
- Can switch between two tasks
- Each task maintains separate state
- No corruption of registers/stack

---

### 4.3 Simple Scheduler
**What**: Round-robin task scheduler

**Implementation**:
- Maintain ready queue
- Timer interrupt triggers scheduler
- Pick next task to run
- Preemptive multitasking

**Success criteria**:
- Multiple tasks run concurrently
- Each gets fair CPU time
- No deadlocks or starvation

---

## Phase 5: Syscall Interface (Week 7)

### 5.1 Syscall Mechanism
**What**: User-to-kernel transition

**Implementation**:
- Use `syscall`/`sysret` instructions (or `int 0x80`)
- Define syscall numbers
- Syscall dispatcher
- Parameter passing (registers)

**Files to create**:
- `kernel/src/syscall/mod.rs` - Syscall subsystem
- `kernel/src/syscall/dispatcher.rs` - Syscall handler
- `kernel/src/syscall/numbers.rs` - Syscall number definitions

**Success criteria**:
- Can invoke syscalls from "userspace" (simulated)
- Parameters passed correctly
- Return values work

---

### 5.2 Core Syscalls
**What**: Implement essential syscalls

**Syscalls to implement**:
- `sys_write(fd, buf, len)` - Write to console/serial
- `sys_read(fd, buf, len)` - Read from keyboard
- `sys_exit(code)` - Terminate task
- `sys_fork()` - Create new task (later)
- `sys_exec(path)` - Load program (later)

**Success criteria**:
- `sys_write` outputs to serial/framebuffer
- `sys_read` gets keyboard input
- `sys_exit` terminates task cleanly

---

## Phase 6: Simple Shell/REPL (Week 8-9)

### 6.1 Text Output
**What**: Console/terminal emulator

**Implementation**:
- Text mode framebuffer rendering
- Font rendering (use embedded bitmap font)
- Scrolling
- Cursor management

**Files to create**:
- `kernel/src/console/mod.rs` - Console subsystem
- `kernel/src/console/font.rs` - Bitmap font data
- `kernel/src/console/terminal.rs` - Terminal emulator

**Dependencies**:
- Consider `noto-sans-mono-bitmap` crate for fonts

**Success criteria**:
- Can print text to screen
- Text scrolls when screen fills
- Cursor visible and movable

---

### 6.2 Command Parser
**What**: Parse user input into commands

**Implementation**:
- Read line from keyboard
- Tokenize input
- Command dispatch table
- Built-in commands: `help`, `clear`, `echo`, `mem`, `tasks`

**Files to create**:
- `kernel/src/shell/mod.rs` - Shell subsystem
- `kernel/src/shell/parser.rs` - Command parser
- `kernel/src/shell/builtins.rs` - Built-in commands

**Success criteria**:
- Can type commands
- Commands execute
- Output displays on screen

---

### 6.3 Rust REPL (Proof of Concept)
**What**: Execute Rust code at runtime

**Implementation**:
- **Simple approach**: Pre-define Rust functions, call by name
- **Advanced approach**: Embed `miri` or JIT compiler (very complex)
- For now: Command-based interface that feels like a REPL

**Example**:
```
> mem
Available: 512 MB
Used: 2 MB

> tasks
Task 0: Running (kernel)
Task 1: Ready (shell)

> echo Hello Polyglot OS!
Hello Polyglot OS!
```

**Success criteria**:
- Interactive shell works
- Can inspect system state
- Foundation for language runtimes

---

## Phase 7: Filesystem (Week 10-11)

### 7.1 VFS Layer
**What**: Virtual filesystem abstraction

**Implementation**:
- Define VFS traits (File, Directory, Filesystem)
- Mount point management
- Path resolution

**Files to create**:
- `kernel/src/fs/mod.rs` - Filesystem subsystem
- `kernel/src/fs/vfs.rs` - VFS layer

---

### 7.2 FAT32 Driver
**What**: Read boot partition

**Implementation**:
- Parse FAT32 structures
- Read files from ESP
- Directory listing
- Consider using `fatfs` crate

**Success criteria**:
- Can read files from boot partition
- Can load programs from disk

---

## Phase 8: Language Runtime Integration (Week 12+)

### 8.1 RustPython Integration
**What**: Embed Python interpreter

**Implementation**:
- Add `rustpython` crate (or `rustpython-vm`)
- Initialize Python VM
- Expose syscalls to Python
- Create Python standard library bindings

**Challenges**:
- RustPython needs `std` - may need to patch for `no_std`
- Large binary size
- Memory requirements

**Alternative**: Consider `micropython` (C-based, smaller)

---

### 8.2 Polyglot Shell
**What**: Multi-language REPL

**Implementation**:
- Detect language from prompt (`:rust`, `:python`, `:zig`)
- Route input to appropriate runtime
- Share data between languages

**Example**:
```
polyglot> :python
Python 3.11 on Polyglot OS
>>> print("Hello from Python!")
Hello from Python!
>>> import os
>>> os.write(1, b"Direct syscall!\n")
Direct syscall!

polyglot> :rust
Rust on Polyglot OS
>> let x = vec![1, 2, 3];
>> println!("{:?}", x);
[1, 2, 3]
```

---

## Dependencies to Add

Update `kernel/Cargo.toml`:
```toml
[dependencies]
limine = "0.5.0"
spin = "0.9"  # Spinlocks for synchronization
x86_64 = "0.15"  # x86_64 structures and helpers
linked_list_allocator = "0.10"  # Heap allocator
lazy_static = { version = "1.4", features = ["spin_no_std"] }  # Static initialization
bitflags = "2.4"  # Bit flag management
noto-sans-mono-bitmap = { version = "0.3", optional = true }  # Font rendering

[features]
default = []
graphics = ["noto-sans-mono-bitmap"]
```

---

## Testing Strategy

### Per-Phase Testing
- **Unit tests**: Test individual components in isolation
- **Integration tests**: Test component interactions
- **QEMU testing**: Boot and verify in emulator
- **Serial logging**: Debug output for all major operations

### Regression Testing
- Keep a test suite that runs on each build
- Automated QEMU tests with expected output

---

## Milestones

- **Milestone 1** (Week 2): Dynamic memory allocation works
- **Milestone 2** (Week 3): Interrupts and exceptions handled
- **Milestone 3** (Week 4): Timer and keyboard functional
- **Milestone 4** (Week 6): Multitasking works
- **Milestone 5** (Week 7): Syscalls implemented
- **Milestone 6** (Week 9): Interactive shell works
- **Milestone 7** (Week 11): Can load programs from disk
- **Milestone 8** (Week 12+): First language runtime embedded

---

## Next Immediate Steps

1. **Start Phase 1.1**: Implement physical memory allocator
2. **Update dependencies**: Add crates to `Cargo.toml`
3. **Fix target**: Use `x86_64-polyglot` custom target consistently
4. **Improve panic handler**: Print panic info to serial

**Ready to start Phase 1.1?** I can begin implementing the physical memory allocator right now.
