#!/bin/bash

# Bootstrap script for Jamos OS (C++ version)
# This script builds the kernel and runs it in QEMU

set -e

echo "Building Jamos kernel (C++)..."
make clean
make all

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
