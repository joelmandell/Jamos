# Jamos
Experimental operating system written in Rust for ARM64/AArch64 architecture.

## Features
- Bare-metal Rust kernel (no_std)
- Targets generic ARM64/AArch64 architecture (tested under QEMU on Apple Silicon Macs)
- Basic bootloader with assembly boot stub for proper stack initialization
- Prints "Hello lovely Anna!" to PL011 UART console with proper FIFO polling
- Runs in QEMU emulator (not intended to boot directly on real M1 hardware due to proprietary boot chain)

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
- `src/main.rs` - Main kernel code with UART driver
- `linker.ld` - Linker script for ARM64
- `.cargo/config.toml` - Cargo configuration for cross-compilation
- `bootstrap.sh` - Build and run script

## Architecture
- **Target**: aarch64-unknown-none (bare-metal ARM64)
- **Machine**: QEMU virt machine
- **CPU**: Cortex-A57
- **Load Address**: 0x40000000
- **UART**: PL011 UART at 0x09000000

## License
Experimental project
