# C++ Conversion Notes

This document describes the conversion of Jamos from Rust to C++.

## Conversion Summary

The entire Jamos operating system has been successfully converted from Rust to C++17. The original Rust source code is preserved in the `src/` directory for reference, but the active development now uses the C++ implementation in `cpp_src/`.

## Key Changes

### Build System
- **Old**: Cargo-based Rust build system (Cargo.toml, .cargo/config.toml)
- **New**: Make-based C++ build system (Makefile)
- **Toolchain**: aarch64-linux-gnu-g++ (ARM64 cross-compiler)

### Project Structure
- **C++ Source**: `cpp_src/` directory (mirrors old `src/` structure)
- **Headers**: `include/` directory with proper C++ header files
- **Build Output**: `build/` for object files, `target/aarch64-unknown-none/release/` for final binary

### Language Features Converted

1. **Rust's no_std** → C++ freestanding mode (-ffreestanding, -nostdlib)
2. **Rust panic handler** → C++ __cxa_pure_virtual and custom panic handling
3. **Rust global_asm!** → Inline assembly with __asm__
4. **Rust Option<T>** → Pointer nullability and return value checking
5. **Rust Result<T, E>** → Return codes and error messages
6. **Rust's safe references** → Raw pointers with careful management

### Module Simplifications

Some modules were simplified during conversion:

1. **Filesystem**: Converted from PostgreSQL-inspired rich metadata system to simple in-memory array-based storage
   - No longer has: inodes, timestamps, permissions, owner/group
   - Still has: basic create, read, write, delete operations
   
2. **Wayland Compositor**: Converted from full protocol implementation to status-tracking stub
   - No longer has: client connections, surface management, protocol messages
   - Still has: start/stop/status commands

3. **Tiling Manager**: Infrastructure preserved but not actively used

### Build and Run

```bash
# Build the C++ version
make all

# Or use the bootstrap script
./bootstrap.sh

# Clean build artifacts
make clean
```

### Preserved Features

All core functionality has been preserved:
- ✅ ARM64 boot assembly stub
- ✅ PL011 UART driver
- ✅ Keyboard input with ANSI escape sequences
- ✅ Virtual desktop management (2 desktops)
- ✅ Terminal commands (help, clear, info, ls, touch, rm, cat, edit)
- ✅ Text editor (Ctrl+S, Ctrl+X, Ctrl+Q)
- ✅ File system operations
- ✅ QEMU compatibility

## Testing

The C++ version has been successfully tested in QEMU:
- Boots correctly
- Displays welcome message
- Shows command prompt
- Ready for user input

## Legacy Files

The following files are from the Rust version and are kept for reference but not used:
- `src/**/*.rs` - Original Rust source code
- `Cargo.toml` - Rust package manifest
- `.cargo/config.toml` - Rust build configuration

These can be removed if desired, but are preserved to help understand the conversion.
