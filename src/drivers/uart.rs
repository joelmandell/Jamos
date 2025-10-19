// PL011 UART driver for ARM64 QEMU virt machine

const UART0_BASE: usize = 0x0900_0000;
const UART0_DR: *mut u32 = UART0_BASE as *mut u32;           // Data register
const UART0_FR: *mut u32 = (UART0_BASE + 0x18) as *mut u32; // Flag register

// UART Flag Register bits
const UART_FR_TXFF: u32 = 1 << 5; // Transmit FIFO full
const UART_FR_RXFE: u32 = 1 << 4; // Receive FIFO empty

#[derive(Clone, Copy)]
pub struct Uart;

impl Uart {
    pub const fn empty() -> Self {
        Uart
    }
    
    pub fn new() -> Self {
        Uart
    }

    pub fn putc(&self, c: u8) {
        unsafe {
            // Wait for TX FIFO to have space
            while (core::ptr::read_volatile(UART0_FR) & UART_FR_TXFF) != 0 {
                // Spin while transmit FIFO is full
            }
            
            // Write character to data register
            core::ptr::write_volatile(UART0_DR, c as u32);
        }
    }

    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            self.putc(byte);
        }
    }

    pub fn getc(&self) -> Option<u8> {
        unsafe {
            // Check if RX FIFO has data
            if (core::ptr::read_volatile(UART0_FR) & UART_FR_RXFE) != 0 {
                None
            } else {
                // Read character from data register
                Some((core::ptr::read_volatile(UART0_DR) & 0xFF) as u8)
            }
        }
    }
}
