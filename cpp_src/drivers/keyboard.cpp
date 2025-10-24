#include "drivers/keyboard.hpp"

bool Keyboard::poll(KeyEvent& event) {
    bool has_data;
    uint8_t c = uart.getc(has_data);
    if (!has_data) {
        return false;
    }

    // Handle ANSI escape sequences for arrow keys
    // ESC[C = Right, ESC[D = Left, ESC[1;5C = Ctrl+Right, ESC[1;5D = Ctrl+Left
    switch (escape_sequence) {
        case EscapeSequence::None:
            if (c == 0x1B) {  // ESC
                escape_sequence = EscapeSequence::Escape;
                return false;
            }
            return process_char(c, event);

        case EscapeSequence::Escape:
            if (c == '[') {
                escape_sequence = EscapeSequence::Bracket;
                return false;
            } else {
                escape_sequence = EscapeSequence::None;
                event.key = Key::Escape;
                event.char_value = 0;
                event.meta = false;
                event.ctrl = false;
                event.shift = false;
                return true;
            }

        case EscapeSequence::Bracket:
            // Check if this is an extended sequence (ESC[1;5C for Ctrl+Right)
            if (c == '1') {
                escape_sequence = EscapeSequence::BracketOne;
                return false;
            }
            
            escape_sequence = EscapeSequence::None;
            switch (c) {
                case 'A': event.key = Key::Up; break;
                case 'B': event.key = Key::Down; break;
                case 'C': event.key = Key::Right; break;
                case 'D': event.key = Key::Left; break;
                default: event.key = Key::Unknown; break;
            }
            event.char_value = 0;
            event.meta = meta_pressed;
            event.ctrl = false;
            event.shift = false;
            return true;

        case EscapeSequence::BracketOne:
            // Expecting ';' after '1'
            if (c == ';') {
                escape_sequence = EscapeSequence::BracketOneColon;
                return false;
            } else {
                // Not a recognized extended sequence, treat as regular key
                escape_sequence = EscapeSequence::None;
                return process_char(c, event);
            }

        case EscapeSequence::BracketOneColon:
            // Expecting '5' for Ctrl modifier
            if (c == '5') {
                escape_sequence = EscapeSequence::BracketOneColonFive;
                return false;
            } else {
                // Not a Ctrl modifier, treat as regular key
                escape_sequence = EscapeSequence::None;
                return process_char(c, event);
            }

        case EscapeSequence::BracketOneColonFive:
            // Final character - arrow key with Ctrl
            escape_sequence = EscapeSequence::None;
            switch (c) {
                case 'A': event.key = Key::Up; break;
                case 'B': event.key = Key::Down; break;
                case 'C': event.key = Key::Right; break;
                case 'D': event.key = Key::Left; break;
                default: event.key = Key::Unknown; break;
            }
            event.char_value = 0;
            event.meta = meta_pressed;
            event.ctrl = true;
            event.shift = false;
            return true;
    }
    
    return false;
}

bool Keyboard::process_char(uint8_t c, KeyEvent& event) {
    // Handle Ctrl+key combinations (Ctrl+A = 0x01, etc.)
    if (c < 0x20) {
        if (c == 0x0A || c == 0x0D) {  // Enter (LF or CR)
            event.key = Key::Enter;
            event.char_value = 0;
            event.meta = false;
            event.ctrl = false;
            event.shift = false;
            return true;
        } else if (c == 0x7F || c == 0x08) {  // Backspace or DEL
            event.key = Key::Backspace;
            event.char_value = 0;
            event.meta = false;
            event.ctrl = false;
            event.shift = false;
            return true;
        } else {
            // Other control characters
            event.key = Key::Char;
            event.char_value = c;
            event.meta = false;
            event.ctrl = true;
            event.shift = false;
            return true;
        }
    }

    // Regular printable character
    event.key = Key::Char;
    event.char_value = c;
    event.meta = meta_pressed;
    event.ctrl = false;
    event.shift = (c >= 'A' && c <= 'Z');
    return true;
}
