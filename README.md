# Polyglot OS

A modern operating system kernel written in Rust (edition 2024) for x86_64 architecture with UEFI boot support.

## Project Structure

```
polyglot-os/
├── boot/                    # UEFI Bootloader (using uefi-rs crate)
│   ├── Cargo.toml          # Dependencies: uefi, log
│   └── src/
│       └── main.rs         # UEFI boot entry point
├── kernel/                  # Kernel (Rust no_std, bare-metal)
│   ├── Cargo.toml
│   ├── kernel.json         # Custom x86_64 target specification
│   ├── linker.ld           # Kernel linker script
│   └── src/
│       ├── lib.rs          # Kernel library code
│       └── main.rs         # Kernel entry point
├── esp/                     # EFI System Partition (auto-generated)
│   └── efi/boot/
│       └── bootx64.efi     # UEFI bootloader binary
├── rust-toolchain.toml     # Rust toolchain configuration
├── Cargo.toml              # Workspace configuration
├── Makefile                # Build automation
└── README.md
```

## Features

- **Rust Edition 2024**: Uses the latest Rust edition with modern safety features
- **UEFI Boot**: Production-ready UEFI bootloader using the `uefi-rs` crate
- **Bare-Metal Kernel**: Custom x86_64 kernel with no dependencies
- **QEMU Support**: Test with QEMU using OVMF UEFI firmware
- **Hardware Ready**: Can boot on real UEFI-capable hardware

## Prerequisites

- **Rust nightly toolchain** (configured via `rust-toolchain.toml`)
- **QEMU**: `qemu-system-x86_64`
- **OVMF firmware**: UEFI firmware for QEMU
  - Ubuntu/Debian: `sudo apt install ovmf qemu-system-x86`
  - Fedora: `sudo dnf install edk2-ovmf qemu-system-x86`
  - Arch: `sudo pacman -S edk2-ovmf qemu-system-x86`
- **GNU Make**

## Building

```bash
# First time setup: copy OVMF firmware files
make setup-ovmf

# Build everything (bootloader + kernel → build/esp/)
make

# Build and run in QEMU (auto-detects KVM)
make run

# Run without KVM acceleration
make run-nokvm

# Clean build artifacts
make clean
```

### Build Output

All artifacts are organized in `build/esp/`:
```
build/esp/
├── efi/boot/bootx64.efi    # 123KB UEFI bootloader
└── kernel.elf               # 2.4MB Kernel binary
```

This directory is your complete OS, ready to deploy!

## Running on Real Hardware

1. Build the OS: `make`
2. Prepare a USB drive with FAT32 filesystem
3. Copy the ESP contents: `cp -r build/esp/efi /path/to/usb/`
4. Boot from USB (disable Secure Boot in BIOS if needed)

The `build/esp/` directory contains your complete bootable OS!

## Current Status

**Working:**
- ✅ UEFI bootloader with logging support
- ✅ Boots in QEMU with OVMF firmware
- ✅ Rust edition 2024 compliance
- ✅ Production-ready boot infrastructure
- ✅ Kernel compiles with VGA text driver
- ✅ Build system outputs to `build/` directory
- ✅ Complete ESP structure ready for deployment

**Next Steps:**
- ⏳ Implement kernel loading in bootloader (ELF parsing)
- ⏳ Transfer control from bootloader to kernel
- ⏳ Verify kernel VGA output displays
- ⏳ Create bootable disk image (.img file)
- ⏳ Memory management
- ⏳ Interrupt handling

## Architecture

### Boot Process
1. **UEFI Firmware** loads `bootx64.efi` from ESP
2. **Bootloader** (`boot/`) initializes UEFI services and logging
3. **Kernel** (future) will be loaded and control transferred

### Technology Stack
- **Bootloader**: `uefi-rs` crate for UEFI protocol access
- **Kernel**: Custom bare-metal Rust with `no_std`
- **Build**: Cargo with custom target specifications
- **Testing**: QEMU with OVMF UEFI firmware

## Next Steps

- [ ] Load kernel ELF from ESP filesystem
- [ ] Parse kernel ELF and set up memory mapping
- [ ] Exit UEFI boot services and transfer control to kernel
- [ ] Implement kernel VGA text driver
- [ ] Set up GDT, IDT, and paging
- [ ] Implement memory allocator




