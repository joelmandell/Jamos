#ifndef DRIVERS_KEYBOARD_HPP
#define DRIVERS_KEYBOARD_HPP

#include "drivers/uart.hpp"
#include <cstdint>

enum class Key {
    Char,
    Enter,
    Backspace,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Meta,
    Unknown
};

struct KeyEvent {
    Key key;
    uint8_t char_value;  // Only valid when key == Key::Char
    bool meta;
    bool ctrl;
    bool shift;
};

enum class EscapeSequence {
    None,
    Escape,
    Bracket,
    BracketOne,
    BracketOneColon,
    BracketOneColonFive
};

class Keyboard {
private:
    Uart uart;
    bool meta_pressed;
    EscapeSequence escape_sequence;
    
    bool process_char(uint8_t c, KeyEvent& event);

public:
    Keyboard() : meta_pressed(false), escape_sequence(EscapeSequence::None) {}
    
    bool poll(KeyEvent& event);
};

#endif // DRIVERS_KEYBOARD_HPP
