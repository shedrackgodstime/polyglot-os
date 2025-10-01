# Polyglot OS Makefile

TARGET      = x86_64-unknown-none
KERNEL_BIN  = build/kernel.elf
IMAGE       = build/polyglot-os.img
CARGO       = cargo +nightly
LIMINE_DIR  = build/limine

.PHONY: all clean run setup-limine image

all: $(IMAGE)

# Build the kernel ELF
$(KERNEL_BIN):
	@mkdir -p build
	$(CARGO) build -Zbuild-std=core,alloc,compiler_builtins \
	    --target $(TARGET) --release
	cp target/$(TARGET)/release/kernel $(KERNEL_BIN)

# Download Limine bootloader
setup-limine:
	@if [ ! -d "$(LIMINE_DIR)" ]; then \
		echo "Downloading Limine..."; \
		git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1 $(LIMINE_DIR); \
	fi
	@if [ ! -f "$(LIMINE_DIR)/limine" ]; then \
		echo "Compiling Limine installer..."; \
		gcc $(LIMINE_DIR)/limine.c -o $(LIMINE_DIR)/limine; \
	fi

# Create bootable disk image using mtools (no sudo required)
image: $(KERNEL_BIN) setup-limine
	@echo "Creating bootable disk image..."
	@rm -f $(IMAGE)
	@dd if=/dev/zero of=$(IMAGE) bs=1M count=64 status=none
	@echo "Creating partition table..."
	@parted -s $(IMAGE) mklabel gpt
	@parted -s $(IMAGE) mkpart primary fat32 2048s 100%
	@parted -s $(IMAGE) set 1 boot on
	@echo "Creating FAT32 filesystem..."
	@mformat -i $(IMAGE)@@1M -F -v POLYGLOT ::
	@echo "Copying kernel and bootloader files..."
	@mmd -i $(IMAGE)@@1M ::/boot
	@mmd -i $(IMAGE)@@1M ::/boot/limine
	@mmd -i $(IMAGE)@@1M ::/EFI
	@mmd -i $(IMAGE)@@1M ::/EFI/BOOT
	@mcopy -i $(IMAGE)@@1M $(KERNEL_BIN) ::/boot/kernel.elf
	@mcopy -i $(IMAGE)@@1M boot/limine.conf ::/boot/limine/limine.conf
	@mcopy -i $(IMAGE)@@1M $(LIMINE_DIR)/limine-bios.sys ::/boot/limine/limine-bios.sys 2>/dev/null || true
	@mcopy -i $(IMAGE)@@1M $(LIMINE_DIR)/BOOTX64.EFI ::/EFI/BOOT/BOOTX64.EFI 2>/dev/null || true
	@echo "Installing Limine bootloader to MBR..."
	@$(LIMINE_DIR)/limine bios-install $(IMAGE) 2>/dev/null || echo "Note: BIOS bootloader install may need manual verification"
	@echo "✓ Bootable image created: $(IMAGE)"

$(IMAGE): image

# Run in QEMU
run: $(IMAGE)
	@echo "Starting Polyglot OS in QEMU..."
	qemu-system-x86_64 -drive format=raw,file=$(IMAGE) -m 256M -serial stdio

# Clean everything
clean:
	$(CARGO) clean
	rm -rf build
