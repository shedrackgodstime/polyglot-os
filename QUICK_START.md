# Quick Start Guide - Polyglot OS

## First Time Setup

```bash
# 1. Setup OVMF firmware (one-time)
make setup-ovmf

# 2. Build the OS
make

# 3. Run in QEMU
make run
```

## Your OS is in `build/esp/`

```
build/esp/
├── efi/boot/bootx64.efi    ← Your UEFI bootloader
└── kernel.elf               ← Your kernel
```

## Common Commands

| Command | What it does |
|---------|--------------|
| `make` | Build bootloader + kernel |
| `make run` | Build and run in QEMU |
| `make clean` | Clean all build artifacts |
| `make boot` | Build only bootloader |
| `make kernel` | Build only kernel |

## Deploy to USB

```bash
# 1. Build
make

# 2. Format USB as FAT32

# 3. Copy files
cp -r build/esp/efi /path/to/usb/

# 4. Boot from USB (disable Secure Boot)
```

## Project Structure

```
polyglot-os/
├── boot/              ← UEFI bootloader (Rust)
├── kernel/            ← Kernel (Rust no_std)
├── build/esp/         ← Your complete OS (generated)
├── Makefile           ← Build automation
└── README.md          ← Full documentation
```

## What's Working

✅ UEFI bootloader boots in QEMU  
✅ Kernel compiles with VGA driver  
✅ Rust edition 2024 compliant  
✅ Build outputs to `build/` directory  
✅ Ready to copy to USB  

## What's Next

The bootloader currently just displays messages. Next step is to make it actually load and execute the kernel!

## Troubleshooting

**QEMU won't start?**
- Run `make setup-ovmf` first
- Check if OVMF is installed: `ls /usr/share/OVMF/`

**KVM error?**
- It's optional, QEMU will run without it
- Makefile auto-detects and falls back

**Build errors?**
- Make sure you have Rust nightly
- Run `rustup update nightly`

## File Sizes

- Bootloader: 123KB
- Kernel: 2.4MB (includes debug symbols)
- Total: ~2.5MB

## Learn More

- `README.md` - Full documentation
- `BUILD_STATUS.md` - Detailed build information
- `UEFI_MIGRATION.md` - Why we use UEFI
