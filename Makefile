# Makefile for Jamos C++ OS

# Toolchain
CXX := clang++
OBJCOPY := llvm-objcopy-18

# Directories
SRC_DIR := cpp_src
INC_DIR := include
BUILD_DIR := build
TARGET_DIR := target/aarch64-unknown-none/release

# Output files
KERNEL_ELF := $(BUILD_DIR)/jamos.elf
KERNEL_BIN := $(TARGET_DIR)/jamos.bin

# Compiler flags
CXXFLAGS := --target=aarch64-linux-gnu \
            -ffreestanding \
            -nostdlib \
            -fno-exceptions \
            -fno-rtti \
            -fno-use-cxa-atexit \
            -Wall \
            -Wextra \
            -Wno-new-returns-null \
            -Wno-nonnull \
            -Wno-unused-command-line-argument \
            -O2 \
            -I$(INC_DIR) \
            -mcpu=cortex-a57 \
            -std=c++17

# Linker flags
LDFLAGS := --target=aarch64-linux-gnu \
           -T linker.ld \
           -nostdlib \
           -Wl,--nmagic

# Source files
CPP_SOURCES := $(shell find $(SRC_DIR) -name '*.cpp')
OBJECTS := $(patsubst $(SRC_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(CPP_SOURCES))

# Phony targets
.PHONY: all clean directories

# Default target
all: directories $(KERNEL_BIN)

# Create build directories
directories:
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(BUILD_DIR)/drivers
	@mkdir -p $(BUILD_DIR)/terminal
	@mkdir -p $(BUILD_DIR)/filesystem
	@mkdir -p $(BUILD_DIR)/editor
	@mkdir -p $(BUILD_DIR)/wayland
	@mkdir -p $(TARGET_DIR)

# Link kernel ELF
$(KERNEL_ELF): $(OBJECTS)
	$(CXX) $(LDFLAGS) -o $@ $^

# Convert ELF to binary
$(KERNEL_BIN): $(KERNEL_ELF)
	$(OBJCOPY) --binary-architecture=aarch64 $< -O binary $@

# Compile C++ sources
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean build artifacts
clean:
	rm -rf $(BUILD_DIR)
	rm -f $(KERNEL_BIN)

# Help target
help:
	@echo "Jamos C++ OS Makefile"
	@echo "Available targets:"
	@echo "  all      - Build the kernel (default)"
	@echo "  clean    - Remove build artifacts"
	@echo "  help     - Show this help message"
