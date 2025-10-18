#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Assembly boot stub to initialize stack pointer before calling Rust
global_asm!(
    r#"
    .section .text.boot
    .global _start
    
_start:
    // Set stack pointer to the end of the stack
    ldr x30, =__stack_end
    mov sp, x30
    
    // Clear BSS section
    ldr x0, =__bss_start
    ldr x1, =__bss_end
clear_bss:
    cmp x0, x1
    b.ge clear_bss_done
    str xzr, [x0], #8
    b clear_bss
clear_bss_done:
    
    // Branch to Rust main
    bl rust_main
    
    // In case rust_main returns, loop forever
hang:
    wfe
    b hang
    "#
);

// PL011 UART base address for QEMU virt machine (ARM64)
// This is the standard UART0 address in QEMU's ARM virt machine
const UART0_BASE: usize = 0x0900_0000;
const UART0_DR: *mut u32 = UART0_BASE as *mut u32;           // Data register
const UART0_FR: *mut u32 = (UART0_BASE + 0x18) as *mut u32; // Flag register

// UART Flag Register bits
const UART_FR_TXFF: u32 = 1 << 5; // Transmit FIFO full

// Assembly boot stub calls this function after setting up the stack
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        // Write message to UART
        let message = b"Hello lovely Anna!\n";
        for &byte in message {
            uart_putc(byte);
        }
    }
    
    // Halt the CPU
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

#[inline(always)]
unsafe fn uart_putc(c: u8) {
    // Wait for TX FIFO to have space
    while (core::ptr::read_volatile(UART0_FR) & UART_FR_TXFF) != 0 {
        // Spin while transmit FIFO is full
    }
    
    // Write character to data register (32-bit register, byte in lower 8 bits)
    core::ptr::write_volatile(UART0_DR, c as u32);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}
