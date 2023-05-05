# Compiler and linker options
ASM = nasm
ASM_FLAGS = -f elf32
LD = ld
LD_FLAGS = -m elf_i386

# Directory and file names
ASM_DIR = asm
SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin
BOOT_IMG = boot.img

# Source files
ASM_SRC = $(wildcard $(ASM_DIR)/*.asm)
ASM_OBJ = $(patsubst $(ASM_DIR)/%.asm, $(OBJ_DIR)/%.o, $(ASM_SRC))
RUST_SRC = $(wildcard $(SRC_DIR)/*.rs)
RUST_OBJ = $(patsubst $(SRC_DIR)/%.rs, $(OBJ_DIR)/%.o, $(RUST_SRC))

# Default target
all: $(BIN_DIR)/$(BOOT_IMG)

# Create the disk image
$(BIN_DIR)/$(BOOT_IMG): $(ASM_OBJ) $(RUST_OBJ)
    objcopy -O binary $(OBJ_DIR)/boot_stage1.o $(OBJ_DIR)/boot_stage1.bin
    objcopy -O binary $(OBJ_DIR)/boot_stage2.o $(OBJ_DIR)/boot_stage2.bin
    objcopy -O binary $(RUST_OBJ)/moss $(RUST_OBJ)/moss.bin
    cat $(OBJ_DIR)/boot_stage1.bin $(OBJ_DIR)/boot_stage2.bin $(RUST_OBJ)/moss.bin > $(BIN_DIR)/$(BOOT_IMG)

# Compile the assembly source files
$(OBJ_DIR)/%.o: $(ASM_DIR)/%.asm
    $(ASM) $(ASM_FLAGS) -o $@ $<

# Compile the Rust source files
$(OBJ_DIR)/%.o: $(SRC_DIR)/%.rs
    rustc --target i686-unknown-linux-gnu -O --emit=obj --crate-type=staticlib --cfg bios -C relocation-model=static -C target-feature=+crt-static -o $@ $<

# Clean up the generated files
clean:
    rm -rf $(OBJ_DIR)/*.o $(OBJ_DIR)/*.bin $(RUST_OBJ)/*.o $(BIN_DIR)/$(BOOT_IMG)

.PHONY: all clean
