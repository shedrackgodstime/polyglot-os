BOOT=boot/target/boot/debug/boot
KERNEL=kernel/target/kernel/debug/kernel
DISK=build/disk.img

all: $(DISK)

$(BOOT):
	cargo +nightly build -Z build-std=core,compiler_builtins --target boot/boot.json -p boot

$(KERNEL):
	cargo +nightly build -Z build-std=core,compiler_builtins,alloc --target kernel/kernel.json -p kernel

$(DISK): $(BOOT) $(KERNEL)
	mkdir -p build
	cat $(BOOT) $(KERNEL) > $(DISK)

run: all
	qemu-system-x86_64 -drive format=raw,file=$(DISK)

clean:
	cargo clean
	rm -rf build
