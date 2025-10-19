# Wayland Implementation Testing Guide

## Overview
This document describes how to test the Wayland compositor implementation in Jamos.

## Building and Running

```bash
./bootstrap.sh
```

Or manually:
```bash
cargo build --release
rust-objcopy --binary-architecture=aarch64 \
    target/aarch64-unknown-none/release/jamos \
    -O binary \
    target/aarch64-unknown-none/release/jamos.bin
qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -kernel target/aarch64-unknown-none/release/jamos.bin \
    -nographic \
    -serial mon:stdio
```

## Testing Wayland Commands

Once the system boots, you'll see the Jamos terminal prompt: `[Desktop 1]$`

### 1. Check Wayland Help
```
wayland
```
Expected output:
```
Usage: wayland [start|stop|status]
  start  - Start the Wayland compositor
  stop   - Stop the Wayland compositor
  status - Show compositor status (default)
```

### 2. Check Initial Status
```
wayland status
```
Expected output:
```
=== Wayland Compositor Status ===
State: Stopped
Connected clients: 0
Active surfaces: 0
Registered globals: 3
```

### 3. Start the Compositor
```
wayland start
```
Expected output:
```
=== Wayland Compositor Started ===
Compositor state: Running
Listening for client connections...

Global interfaces registered:
  - wl_compositor (version 4)
  - wl_seat (version 7)
  - wl_output (version 3)

Use 'wayland status' to check compositor status
Use 'wayland stop' to stop the compositor
```

### 4. Check Running Status
```
wayland status
```
Expected output:
```
=== Wayland Compositor Status ===
State: Running
Connected clients: 0
Active surfaces: 0
Registered globals: 3
```

### 5. Stop the Compositor
```
wayland stop
```
Expected output:
```
Wayland compositor stopped.
```

### 6. Verify Stopped
```
wayland status
```
Expected output should show `State: Stopped`

## Implementation Details

The Wayland implementation includes:
- **Compositor Module** (`src/wayland/compositor.rs`): Main compositor logic
- **Protocol Module** (`src/wayland/protocol.rs`): Wayland protocol structures
- **Surface Module** (`src/wayland/surface.rs`): Surface management
- **Integration**: Command handler in `src/main.rs`

### Features Implemented
- Client connection management (up to 8 concurrent clients)
- Surface creation, attachment, commit, and destruction
- Global interface registry (compositor, seat, output)
- Protocol message handling infrastructure
- Manual start/stop control from terminal

### Architecture
- Zero-allocation design suitable for bare-metal OS
- Static arrays for client and surface storage
- Direct integration with VDM and Screen subsystems

## Exit QEMU
Press `Ctrl-A` then `X` to exit QEMU.
