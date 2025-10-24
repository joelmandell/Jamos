#include "terminal/screen.hpp"

void Screen::clear() {
    // Clear screen using ANSI escape codes
    uart.puts("\x1B[2J\x1B[H");
    prompt_shown = false;
}

void Screen::putc(uint8_t c) {
    uart.putc(c);
}

void Screen::puts(const char* s) {
    uart.puts(s);
}

void Screen::render() {
    // No-op for direct output mode
}
