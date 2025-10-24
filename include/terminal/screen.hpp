#ifndef TERMINAL_SCREEN_HPP
#define TERMINAL_SCREEN_HPP

#include "drivers/uart.hpp"

class Screen {
private:
    Uart uart;
    bool prompt_shown;

public:
    Screen() : prompt_shown(false) {}
    
    void clear();
    void putc(uint8_t c);
    void puts(const char* s);
    void render();
};

#endif // TERMINAL_SCREEN_HPP
