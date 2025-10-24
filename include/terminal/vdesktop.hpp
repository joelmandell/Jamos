#ifndef TERMINAL_VDESKTOP_HPP
#define TERMINAL_VDESKTOP_HPP

#include "terminal/screen.hpp"
#include "drivers/uart.hpp"
#include <cstddef>

constexpr size_t MAX_NAME_LEN = 16;
constexpr size_t INPUT_BUFFER_LEN = 32;

class VirtualDesktop {
private:
    uint8_t name[MAX_NAME_LEN];
    size_t name_len;
    Screen screen;
    uint8_t input_buffer[INPUT_BUFFER_LEN];
    size_t input_len;
    bool is_active;

public:
    VirtualDesktop() : name_len(0), input_len(0), is_active(false) {}
    
    void init(const char* name_str);
    void set_name(const char* name_str);
    size_t copy_name_to(uint8_t* buf, size_t buf_len);
    Screen* screen_mut() { return &screen; }
    
    void add_input(uint8_t c);
    void remove_input();
    const uint8_t* get_input() const { return input_buffer; }
    size_t get_input_len() const { return input_len; }
    void clear_input();
    
    bool active() const { return is_active; }
};

class VirtualDesktopManager {
private:
    VirtualDesktop desktop1;
    VirtualDesktop desktop2;
    size_t current_index;

public:
    VirtualDesktopManager() : current_index(0) {}
    
    void init();
    VirtualDesktop* current_mut();
    bool next();
    bool previous();
    size_t get_index() const { return current_index; }
    size_t get_count() const;
};

#endif // TERMINAL_VDESKTOP_HPP
