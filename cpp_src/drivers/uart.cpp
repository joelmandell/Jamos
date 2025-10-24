#include "drivers/uart.hpp"

// PL011 UART registers
constexpr uintptr_t UART0_BASE = 0x09000000;
volatile uint32_t* const UART0_DR = reinterpret_cast<volatile uint32_t*>(UART0_BASE);
volatile uint32_t* const UART0_FR = reinterpret_cast<volatile uint32_t*>(UART0_BASE + 0x18);

// UART Flag Register bits
constexpr uint32_t UART_FR_TXFF = 1 << 5; // Transmit FIFO full
constexpr uint32_t UART_FR_RXFE = 1 << 4; // Receive FIFO empty

void Uart::putc(uint8_t c) const {
    // Wait for TX FIFO to have space
    while ((*UART0_FR & UART_FR_TXFF) != 0) {
        // Spin while transmit FIFO is full
    }
    
    // Write character to data register
    *UART0_DR = c;
}

void Uart::puts(const char* s) const {
    while (*s) {
        putc(*s++);
    }
}

uint8_t Uart::getc(bool& has_data) const {
    // Check if RX FIFO has data
    if ((*UART0_FR & UART_FR_RXFE) != 0) {
        has_data = false;
        return 0;
    } else {
        has_data = true;
        return static_cast<uint8_t>(*UART0_DR & 0xFF);
    }
}
