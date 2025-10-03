# Polyglot OS Makefile (UEFI OVMF)
TARGET      = x86_64-unknown-none
KERNEL_BIN  = build/kernel.elf
IMAGE       = build/polyglot-os.img
CARGO       = cargo +nightly
LIMINE_DIR  = build/limine

.PHONY: all clean distclean run setup-limine image

all: $(IMAGE)

# Build the kernel ELF
$(KERNEL_BIN):
	@mkdir -p build
	$(CARGO) build -Zbuild-std=core,compiler_builtins \
	    --target $(TARGET) --release
	cp target/$(TARGET)/release/kernel $(KERNEL_BIN)

# Download Limine bootloader
setup-limine:
	@if [ ! -d "$(LIMINE_DIR)" ]; then \
		echo "Downloading Limine..."; \
		git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1 $(LIMINE_DIR); \
	fi
	@make -C $(LIMINE_DIR)

# Create bootable disk image using mtools (no sudo required)
image: $(KERNEL_BIN) setup-limine
	@echo "Creating bootable disk image..."
	@rm -f $(IMAGE)
	@dd if=/dev/zero of=$(IMAGE) bs=1M count=64 status=none
	@echo "Creating partition table..."
	@parted -s $(IMAGE) mklabel gpt
	@parted -s $(IMAGE) mkpart primary fat32 2048s 100%
	@parted -s $(IMAGE) set 1 esp on
	@echo "Creating FAT32 filesystem..."
	@mformat -i $(IMAGE)@@1M -F -v POLYGLOT ::
	@echo "Copying kernel and bootloader files..."
	@mmd -i $(IMAGE)@@1M ::/boot
	@mmd -i $(IMAGE)@@1M ::/boot/limine
	@mmd -i $(IMAGE)@@1M ::/EFI
	@mmd -i $(IMAGE)@@1M ::/EFI/BOOT
	@mcopy -i $(IMAGE)@@1M $(KERNEL_BIN) ::/kernel.elf
	@mcopy -i $(IMAGE)@@1M boot/limine.conf ::/boot/limine/limine.conf
	@mcopy -i $(IMAGE)@@1M $(LIMINE_DIR)/BOOTX64.EFI ::/EFI/BOOT/BOOTX64.EFI 2>/dev/null || true
	@mcopy -i $(IMAGE)@@1M $(LIMINE_DIR)/limine-bios.sys ::/boot/limine/limine-bios.sys 2>/dev/null || true
	@echo "Installing Limine BIOS bootloader..."
	@$(LIMINE_DIR)/limine bios-install $(IMAGE) >/dev/null 2>&1 || true
	@echo "✓ Hybrid (UEFI + BIOS) bootable image created: $(IMAGE)"

$(IMAGE): image

# Run in QEMU with OVMF UEFI firmware
run: $(IMAGE)
	@echo "Starting Polyglot OS in QEMU..."
	@if [ -f /usr/share/ovmf/OVMF.fd ]; then \
		echo "Using UEFI (OVMF) - /usr/share/ovmf/OVMF.fd"; \
		qemu-system-x86_64 \
		    -drive format=raw,file=$(IMAGE) \
		    -m 256M \
		    -serial stdio \
		    -no-reboot \
		    -bios /usr/share/ovmf/OVMF.fd \
		    -display gtk,grab-on-hover=off; \
	elif [ -f /usr/share/qemu/OVMF.fd ]; then \
		echo "Using UEFI (OVMF) - /usr/share/qemu/OVMF.fd"; \
		qemu-system-x86_64 \
		    -drive format=raw,file=$(IMAGE) \
		    -m 256M \
		    -serial stdio \
		    -no-reboot \
		    -bios /usr/share/qemu/OVMF.fd \
		    -display gtk,grab-on-hover=off; \
	elif [ -f /usr/share/OVMF/OVMF_CODE.fd ]; then \
		echo "Using UEFI (OVMF) - split code/vars"; \
		qemu-system-x86_64 \
		    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd \
		    -drive if=pflash,format=raw,file=/tmp/OVMF_VARS.fd \
		    -drive format=raw,file=$(IMAGE) \
		    -m 256M \
		    -serial stdio \
		    -no-reboot \
		    -display gtk,grab-on-hover=off; \
	else \
		echo "OVMF not found, falling back to BIOS (SeaBIOS)"; \
		qemu-system-x86_64 \
		    -drive format=raw,file=$(IMAGE) \
		    -m 256M \
		    -serial stdio \
		    -no-reboot \
		    -display gtk,grab-on-hover=off; \
	fi 

# Test .....
test: 





# Clean everything
clean:
	rm -rf build

distclean: clean
	$(CARGO) clean
