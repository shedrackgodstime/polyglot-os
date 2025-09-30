# Paths to build artifacts
BOOT_EFI=target/x86_64-unknown-uefi/debug/boot.efi
KERNEL_BIN=target/kernel/debug/kernel
BUILD_DIR=build
ESP_DIR=$(BUILD_DIR)/esp
OVMF_CODE=/usr/share/OVMF/OVMF_CODE.fd
OVMF_VARS=/usr/share/OVMF/OVMF_VARS.fd

# Rust build flags for kernel (bare-metal)
RUSTFLAGS_KERNEL=-C link-arg=-Tkernel/linker.ld

.PHONY: all boot kernel esp run run-uefi clean setup-ovmf

all: esp

# Build UEFI bootloader
boot:
	@echo "Building UEFI bootloader..."
	@cargo build --target x86_64-unknown-uefi -p boot

# Build kernel (bare-metal)
kernel:
	@echo "Building kernel..."
	@RUSTFLAGS="$(RUSTFLAGS_KERNEL)" cargo +nightly build \
		-Z build-std=core,compiler_builtins,alloc \
		--target kernel/kernel.json \
		-p kernel

# Create ESP (EFI System Partition) directory structure
esp: boot kernel
	@echo "Creating ESP in build/ directory..."
	@mkdir -p $(ESP_DIR)/efi/boot
	@cp $(BOOT_EFI) $(ESP_DIR)/efi/boot/bootx64.efi
	@cp $(KERNEL_BIN) $(ESP_DIR)/kernel.elf
	@echo "✓ Build complete!"
	@echo "  - Bootloader: $(ESP_DIR)/efi/boot/bootx64.efi"
	@echo "  - Kernel:     $(ESP_DIR)/kernel.elf"
	@echo "  - ESP ready:  $(ESP_DIR)/"

# Copy OVMF firmware files (run once)
setup-ovmf:
	@if [ -f $(OVMF_CODE) ]; then \
		cp $(OVMF_CODE) .; \
		cp $(OVMF_VARS) .; \
		echo "OVMF firmware files copied"; \
	else \
		echo "Error: OVMF not found at $(OVMF_CODE)"; \
		echo "Install with: sudo apt install ovmf (Ubuntu/Debian) or sudo dnf install edk2-ovmf (Fedora)"; \
		exit 1; \
	fi

# Run in QEMU with UEFI firmware (auto-detect KVM)
run: esp
	@if [ ! -f OVMF_CODE.fd ]; then \
		echo "OVMF firmware not found. Run 'make setup-ovmf' first."; \
		exit 1; \
	fi
	@if [ -e /dev/kvm ] && [ -r /dev/kvm ] && [ -w /dev/kvm ]; then \
		echo "Running with KVM acceleration..."; \
		qemu-system-x86_64 -enable-kvm \
			-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
			-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
			-drive format=raw,file=fat:rw:$(ESP_DIR) \
			-net none; \
	else \
		echo "KVM not available, running without acceleration..."; \
		qemu-system-x86_64 \
			-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
			-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
			-drive format=raw,file=fat:rw:$(ESP_DIR) \
			-net none; \
	fi

# Alternative: run without KVM (slower but works everywhere)
run-nokvm: esp
	@if [ ! -f OVMF_CODE.fd ]; then \
		echo "OVMF firmware not found. Run 'make setup-ovmf' first."; \
		exit 1; \
	fi
	qemu-system-x86_64 \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
		-drive format=raw,file=fat:rw:$(ESP_DIR) \
		-net none

clean:
	cargo clean
	rm -rf $(ESP_DIR)
	rm -f OVMF_CODE.fd OVMF_VARS.fd
