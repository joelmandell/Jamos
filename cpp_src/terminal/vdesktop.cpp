#include "terminal/vdesktop.hpp"
#include <cstring>

void VirtualDesktop::init(const char* name_str) {
    set_name(name_str);
    is_active = true;
    input_len = 0;
}

void VirtualDesktop::set_name(const char* name_str) {
    name_len = 0;
    while (name_str[name_len] != '\0' && name_len < MAX_NAME_LEN) {
        name[name_len] = name_str[name_len];
        name_len++;
    }
}

size_t VirtualDesktop::copy_name_to(uint8_t* buf, size_t buf_len) {
    size_t len = (name_len < buf_len) ? name_len : buf_len;
    for (size_t i = 0; i < len; i++) {
        buf[i] = name[i];
    }
    return len;
}

void VirtualDesktop::add_input(uint8_t c) {
    if (input_len < INPUT_BUFFER_LEN) {
        input_buffer[input_len++] = c;
    }
}

void VirtualDesktop::remove_input() {
    if (input_len > 0) {
        input_len--;
    }
}

void VirtualDesktop::clear_input() {
    input_len = 0;
}

void VirtualDesktopManager::init() {
    desktop1.init("Desktop 1");
}

VirtualDesktop* VirtualDesktopManager::current_mut() {
    if (current_index == 0 && desktop1.active()) {
        return &desktop1;
    } else if (current_index == 1 && desktop2.active()) {
        return &desktop2;
    }
    return nullptr;
}

bool VirtualDesktopManager::next() {
    if (!desktop2.active() && current_index == 0) {
        // Activate second desktop
        desktop2.init("Desktop 2");
        current_index = 1;
        return true;
    } else if (current_index == 0 && desktop2.active()) {
        // Switch to second desktop
        current_index = 1;
        return true;
    }
    return false;
}

bool VirtualDesktopManager::previous() {
    if (current_index > 0) {
        current_index--;
        return true;
    }
    return false;
}

size_t VirtualDesktopManager::get_count() const {
    return desktop2.active() ? 2 : 1;
}
