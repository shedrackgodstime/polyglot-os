#!/bin/bash
# Quick test script to run QEMU and capture serial output

echo "Running Polyglot OS in QEMU (BIOS mode)..."
echo "Press Ctrl+C to exit"
echo "================================"

qemu-system-x86_64 \
    -drive format=raw,file=build/polyglot-os.img \
    -m 256M \
    -serial stdio \
    -no-reboot \
    -display none
