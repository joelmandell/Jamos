# Jamos
Experimental operating system written in Rust for ARM64/AArch64 architecture.

## Features
- Bare-metal Rust kernel (no_std)
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
- Rust toolchain (rustc, cargo)
- QEMU (qemu-system-aarch64)
- cargo-binutils (for rust-objcopy)
- llvm-tools-preview (Rust component)

## Quick Start

### Install Dependencies

1. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Add ARM64 bare-metal target:
```bash
rustup target add aarch64-unknown-none
```

3. Install cargo-binutils:
```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

4. Install QEMU:
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
1. Build the kernel using Cargo
2. Convert the ELF binary to raw binary format
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
cargo build --release

# Convert to binary
rust-objcopy --binary-architecture=aarch64 \
    target/aarch64-unknown-none/release/jamos \
    -O binary \
    target/aarch64-unknown-none/release/jamos.bin

# Run in QEMU
qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -kernel target/aarch64-unknown-none/release/jamos.bin \
    -nographic \
    -serial mon:stdio
```

## Project Structure
- `src/main.rs` - Main kernel code with terminal loop and command handling
- `src/drivers/` - Hardware drivers (UART, keyboard)
- `src/terminal/` - Virtual desktop management, screen, and tiling
- `src/filesystem/` - PostgreSQL-inspired metadata-rich filesystem
- `src/editor/` - Simple nano-like text editor
- `src/wayland/` - Wayland compositor implementation
- `linker.ld` - Linker script for ARM64
- `.cargo/config.toml` - Cargo configuration for cross-compilation
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
The filesystem uses a PostgreSQL-inspired design with rich metadata:
- **Inodes**: Each file/directory has an inode with metadata (size, timestamps, permissions, owner/group)
- **File entries**: Name-to-inode mappings
- **Data blocks**: Fixed-size blocks (512 bytes) for file content
- **Metadata tracking**: Creation time, modification time, size, file type

### Tiling Manager
Infrastructure is in place for micro-space tiling within virtual desktops, allowing multiple panes to be displayed side-by-side or stacked. This feature is ready for future commands to split and manage panes.

### Wayland Compositor
The Wayland compositor provides a minimal implementation of the Wayland protocol for display server functionality:
- **Client Management**: Support for multiple client connections (up to 8 concurrent clients)
- **Surface Management**: Creation, attachment, commit, and destruction of surfaces
- **Global Registry**: Advertising of compositor, seat, and output interfaces
- **Protocol Messages**: Handling of core Wayland protocol operations
- **Manual Control**: Start and stop the compositor from the terminal with the `wayland` command

## License
Experimental project
