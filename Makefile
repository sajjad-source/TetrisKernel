# Output files
BOOTLOADER1_BIN = bootloader1.bin
RUST_BIN = main.o
DISK_IMAGE = disk_image.bin

# Tools
NASM = nasm
CAT = cat

# Default target
all: $(DISK_IMAGE)

# Compile bootloader1.asm into a binary
$(BOOTLOADER1_BIN): bootloader1.asm
	$(NASM) -f bin -o $@ $<

# Compile the Rust code and generate a binary
$(RUST_BIN): main.rs
	rustup run nightly cargo rustc --target i386-unknown-none.json -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem --release -- --emit=obj -o target/i386-unknown-none/release/$@
	mv target/i386-unknown-none/release/$@ $@

# Concatenate the binary files into a single disk image
$(DISK_IMAGE): $(BOOTLOADER1_BIN) $(RUST_BIN)
	$(CAT) $^ > $@

# Clean up generated files
clean:
	rm -f $(BOOTLOADER1_BIN) $(RUST_BIN) $(DISK_IMAGE)
	rm -rf target/

# Test the bootloader using QEMU
.PHONY: test
test: $(DISK_IMAGE)
	qemu-system-i386 -drive file=$(DISK_IMAGE),format=raw
