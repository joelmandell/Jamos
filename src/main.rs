#![no_std]
#![no_main]

use core::panic::PanicInfo;

// UART base address for QEMU virt machine (ARM64)
const UART0: *mut u8 = 0x0900_0000 as *mut u8;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize UART
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
    // Write character to UART data register
    core::ptr::write_volatile(UART0, c);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}
