polyglot-os/
├── boot/
│   ├── stage1.asm           # BIOS boot sector
│   └── stage2.zig           # Second stage loader
├── kernel/
│   ├── Cargo.toml
│   ├── build.rs             # Build script for kernel
│   └── src/
│       ├── main.rs          # Kernel entry point
│       ├── vga.rs           # VGA text driver
│       ├── memory.rs        # Memory management
│       └── bootinfo.rs      # Boot information structs
├── scripts/
│   └── run_qemu.sh          # QEMU launch script
├── Makefile
└── README.md 




