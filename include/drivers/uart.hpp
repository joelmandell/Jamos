#ifndef DRIVERS_UART_HPP
#define DRIVERS_UART_HPP

#include <cstdint>

// PL011 UART driver for ARM64 QEMU virt machine
class Uart {
public:
    Uart() = default;
    
    void putc(uint8_t c) const;
    void puts(const char* s) const;
    uint8_t getc(bool& has_data) const;
};

#endif // DRIVERS_UART_HPP
