# Polyglot OS --- Vision Notes

## Concept:

Polyglot OS is an experimental operating system where the *programming
languages themselves are the OS*. Instead of an OS that merely hosts
compilers and interpreters, Polyglot OS is built from them. Every layer
of the system --- from kernel to shell --- is written in, and exposed
through, languages like Rust, Zig, Python, C, and Bash.

## Philosophy:

-   The OS is the compiler, and the compiler is the OS.
-   No strict separation between "system" and "programs": all code is
    part of the living OS.
-   Booting into the OS means booting directly into a language
    environment (e.g., Python REPL).
-   The OS is self-hosting: it can be extended, modified, and tested
    from within itself, in real time.

## Goals:

-   Boot a minimal kernel (Rust/Zig).
-   Provide basic drivers and syscalls (memory, I/O, FS).
-   Embed language runtimes directly into the kernel:
    -   Rust runtime for safety and performance.
    -   Zig runtime for low-level utilities.
    -   C runtime for drivers and legacy FFI.
    -   Python runtime (RustPython) for scripting and high-level user
        code.
    -   Bash/shell for scripting convenience.
-   Allow the system to be interactively extended by writing new OS code
    inside the OS itself.

## Non-Goals (initially):

-   Full POSIX compatibility.
-   Full GUI or desktop environment.
-   Heavyweight userspace applications.

## High-Level Architecture:

────────────────────────────── Polyglot OS
────────────────────────────── \[ User Interaction Layer \] - Python
REPL (RustPython) - Rust REPL / Zig REPL - Shell (Bash-like, optional)

────────────────────────────── \[ Language Runtime Layer \] - Rust
runtime (native, kernel-safe) - Zig runtime / tooling - C runtime
(minimal libc / driver FFI) - Python runtime (RustPython embedded)

All runtimes directly talk to kernel syscalls.

────────────────────────────── \[ Kernel Services (written in Rust/Zig)
\] - Memory manager - Process/task scheduler - Interrupts + timers -
Virtual File System - Syscall API exposed to runtimes

────────────────────────────── \[ Low-Level Drivers (written in
Zig/C/Rust) \] - Disk (block device, fs) - Keyboard / Input - Console /
Framebuffer - Network (later) - Other peripherals

────────────────────────────── \[ Boot / Init \] - Stage 1 bootloader
(ASM or Zig) - Stage 2 loader (Rust/Zig) - Load kernel + runtimes - Drop
into REPL ────────────────────────────── \[ Hardware \] CPU \| Memory \|
Disk \| Devices ──────────────────────────────

## Execution Model:

-   Boot → Init kernel → Initialize runtimes → Drop user into REPL.
-   Any code written in REPL (Python, Rust, Zig, etc.) interacts
    directly with kernel syscalls.
-   The OS grows by extending itself with code written inside itself.

## Roadmap:

1.  Bootloader (BIOS/UEFI) --- already working prototype.
2.  Minimal Rust kernel --- console I/O, memory, interrupts.
3.  Keyboard + timer drivers (C/Zig).
4.  Embed RustPython → Boot directly into Python REPL.
5.  Add syscall bindings to Python (os.write, os.read, fs, etc.).
6.  Introduce Zig/Rust REPLs for low-level coding inside the OS.
7.  Expand driver + filesystem support.
8.  Work toward multi-runtime integration and polyglot interoperability.

## Risks & Mitigation:

-   Complexity of maintaining multiple runtimes → Start with Rust +
    Python (RustPython).
-   Toolchain heaviness (Rust/Zig compilers) → Provide stripped-down
    REPLs or interpreters first.
-   Debugging kernel + runtime integration → Use QEMU for testing, layer
    by layer.
-   Feature creep → Stick to minimal roadmap first.

## Overall Vision:

Polyglot OS is not just an operating system --- it is a living, polyglot
environment where the languages ARE the OS. It boots into a programming
environment where Rust, Zig, Python, C, and Bash are first-class
citizens, each capable of extending the system itself. The dream: to
blur the line between operating system, compiler, and runtime into one
unified platform.
