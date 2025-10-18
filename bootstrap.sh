#!/bin/bash

# Bootstrap script for Jamos OS
# This script builds the kernel and runs it in QEMU

set -e

echo "Building Jamos kernel..."
cargo build --release

echo "Creating binary image..."
# Convert ELF to raw binary
rust-objcopy --binary-architecture=aarch64 \
    target/aarch64-unknown-none/release/jamos \
    -O binary \
    target/aarch64-unknown-none/release/jamos.bin

echo "Starting QEMU..."
echo "Press Ctrl-A then X to exit QEMU"
echo ""

# Run in QEMU with ARM64 virt machine
# Using -kernel which properly handles raw binary and device tree placement
qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -kernel target/aarch64-unknown-none/release/jamos.bin \
    -nographic \
    -serial mon:stdio
