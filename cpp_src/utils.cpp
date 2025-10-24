#include "utils.hpp"

void print_number(Screen* screen, size_t n) {
    uint8_t buf[20];
    size_t num = n;
    size_t len = 0;
    
    if (num == 0) {
        buf[0] = '0';
        len = 1;
    } else {
        while (num > 0) {
            buf[len] = '0' + (num % 10);
            num /= 10;
            len++;
        }
    }
    
    // Print in reverse
    for (size_t i = len; i > 0; i--) {
        screen->putc(buf[i - 1]);
    }
}
