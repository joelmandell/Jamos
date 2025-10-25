# Jamos
Experimental operating system written in C++ for ARM64/AArch64 architecture.

## Features
- Bare-metal C++ kernel (freestanding)
- Targets generic ARM64/AArch64 architecture (tested under QEMU on Apple Silicon Macs)
- Basic bootloader with assembly boot stub for proper stack initialization
- Prints "Hello lovely Anna!" to PL011 UART console with proper FIFO polling
- Runs in QEMU emulator (not intended to boot directly on real M1 hardware due to proprietary boot chain)
- **Virtual Desktop Management**: Multiple virtual desktops with switching support
  - Ctrl+Right: Switch to next desktop (creates new if needed)
  - Ctrl+Left: Switch to previous desktop
  - Ctrl+N: Rename current desktop
- **PostgreSQL-inspired Filesystem**: Rich metadata filesystem with inode-based storage
  - Commands: `ls`, `touch <file>`, `rm <file>`, `cat <file>`, `edit <file>`
  - Rich metadata: size, timestamps, permissions, owner/group IDs
  - File operations: create, read, write, delete
- **Text Editor**: Simple nano-like text editor
  - Ctrl+S: Save file
  - Ctrl+Q: Quit without saving
  - Ctrl+X: Save and quit
  - Basic cursor movement with arrow keys
  - Insert and delete operations
- **Tiling Window Manager**: Micro-space tiling within virtual desktops (infrastructure ready)
- **Wayland Compositor**: Minimal Wayland compositor with protocol support
  - Commands: `wayland start`, `wayland stop`, `wayland status`
  - Client connection management
  - Surface management
  - Global interface registry (compositor, seat, output)

## Prerequisites
- Clang/LLVM toolchain (clang-16 or later)
- QEMU (qemu-system-aarch64)
- GNU Make

## Quick Start

### Install Dependencies

1. Install Clang/LLVM toolchain:
```bash
# On Ubuntu/Debian
sudo apt-get install clang llvm

# On macOS
brew install llvm
```

2. Install QEMU:
```bash
# On Ubuntu/Debian
sudo apt-get install qemu-system-aarch64

# On macOS
brew install qemu
```

### Build and Run

Simply run the bootstrap script:
```bash
./bootstrap.sh
```

This will:
1. Build the kernel using Make and Clang (ARM64 cross-compilation)
2. Convert the ELF binary to raw binary format with llvm-objcopy
3. Launch QEMU with the kernel

Press `Ctrl-A` then `X` to exit QEMU.

### Available Commands

Once the OS boots, you'll see a command prompt. Available commands:

- `help` - Show help message
- `clear` - Clear screen
- `info` - Show desktop information
- `ls` - List all files in the filesystem
- `touch <filename>` - Create a new file
- `rm <filename>` - Delete a file
- `cat <filename>` - Display file contents
- `edit <filename>` - Open file in text editor
- `wayland [start|stop|status]` - Control the Wayland compositor

### Keyboard Shortcuts

- **Ctrl+Right**: Switch to next desktop (creates new if at last desktop)
- **Ctrl+Left**: Switch to previous desktop
- **Ctrl+N**: Rename current desktop

In the text editor:
- **Ctrl+S**: Save file
- **Ctrl+Q**: Quit without saving
- **Ctrl+X**: Save and exit
- **Arrow keys**: Move cursor
- **Backspace**: Delete character

### Manual Build

To build manually:
```bash
# Build the kernel
make all

# Run in QEMU
qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -kernel target/aarch64-unknown-none/release/jamos.bin \
    -nographic \
    -serial mon:stdio

# Clean build artifacts
make clean
```

## Project Structure
- `cpp_src/main.cpp` - Main kernel code with terminal loop and command handling
- `cpp_src/drivers/` - Hardware drivers (UART, keyboard)
- `cpp_src/terminal/` - Virtual desktop management, screen, and tiling
- `cpp_src/filesystem/` - Simplified in-memory filesystem
- `cpp_src/editor/` - Simple nano-like text editor
- `cpp_src/wayland/` - Wayland compositor stub
- `include/` - Header files for all modules
- `linker.ld` - Linker script for ARM64
- `Makefile` - Build configuration
- `bootstrap.sh` - Build and run script

## Architecture
- **Target**: aarch64-unknown-none (bare-metal ARM64)
- **Machine**: QEMU virt machine
- **CPU**: Cortex-A57
- **Load Address**: 0x40000000
- **UART**: PL011 UART at 0x09000000

### Virtual Desktop Manager
The system supports multiple virtual desktops, each with its own screen buffer and command history. Desktops can be created on-demand and renamed for easy identification.

### Filesystem Architecture
The filesystem uses a simplified in-memory design:
- **File entries**: Fixed array of file entries with name and data
- **Simple operations**: create, read, write, delete, list
- **In-memory storage**: All data stored in memory (no persistence)

### Tiling Manager
Infrastructure is in place for micro-space tiling within virtual desktops, allowing multiple panes to be displayed side-by-side or stacked. This feature is ready for future commands to split and manage panes.

### Wayland Compositor
The Wayland compositor provides a minimal stub implementation:
- **Basic status tracking**: Start, stop, and status commands
- **Manual control**: Start and stop the compositor from the terminal with the `wayland` command
- **Infrastructure ready**: Can be extended with full protocol support

## License
Experimental project
