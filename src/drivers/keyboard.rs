// Keyboard driver - currently using UART for input in QEMU
// This will be extended to support PS/2 keyboard or virtio-input in the future

use super::uart::Uart;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(u8),
    Enter,
    Backspace,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Meta,       // Win/Super key
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub meta: bool,  // Meta/Win key pressed
    pub ctrl: bool,  // Ctrl key pressed
    pub shift: bool, // Shift key pressed
}

pub struct Keyboard {
    uart: Uart,
    meta_pressed: bool,
    ctrl_pressed: bool,
    escape_sequence: EscapeSequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EscapeSequence {
    None,
    Escape,
    Bracket,
}

impl Keyboard {
    pub fn new(uart: Uart) -> Self {
        Keyboard {
            uart,
            meta_pressed: false,
            ctrl_pressed: false,
            escape_sequence: EscapeSequence::None,
        }
    }

    pub fn poll(&mut self) -> Option<KeyEvent> {
        let c = self.uart.getc()?;

        // Handle ANSI escape sequences for arrow keys
        match self.escape_sequence {
            EscapeSequence::None => {
                if c == 0x1B {  // ESC
                    self.escape_sequence = EscapeSequence::Escape;
                    return None;
                }
                self.process_char(c)
            }
            EscapeSequence::Escape => {
                if c == b'[' {
                    self.escape_sequence = EscapeSequence::Bracket;
                    return None;
                } else {
                    self.escape_sequence = EscapeSequence::None;
                    Some(KeyEvent {
                        key: Key::Escape,
                        meta: false,
                        ctrl: false,
                        shift: false,
                    })
                }
            }
            EscapeSequence::Bracket => {
                self.escape_sequence = EscapeSequence::None;
                let key = match c {
                    b'A' => Key::Up,
                    b'B' => Key::Down,
                    b'C' => Key::Right,
                    b'D' => Key::Left,
                    _ => Key::Unknown,
                };
                Some(KeyEvent {
                    key,
                    meta: self.meta_pressed,
                    ctrl: false,
                    shift: false,
                })
            }
        }
    }

    fn process_char(&mut self, c: u8) -> Option<KeyEvent> {
        // Handle Ctrl+key combinations (Ctrl+A = 0x01, etc.)
        if c < 0x20 {
            match c {
                0x0A | 0x0D => {  // Enter (LF or CR)
                    return Some(KeyEvent {
                        key: Key::Enter,
                        meta: false,
                        ctrl: false,
                        shift: false,
                    });
                }
                0x7F | 0x08 => {  // Backspace or DEL
                    return Some(KeyEvent {
                        key: Key::Backspace,
                        meta: false,
                        ctrl: false,
                        shift: false,
                    });
                }
                _ => {
                    // Other control characters
                    return Some(KeyEvent {
                        key: Key::Char(c),
                        meta: false,
                        ctrl: true,
                        shift: false,
                    });
                }
            }
        }

        // Regular printable character
        Some(KeyEvent {
            key: Key::Char(c),
            meta: self.meta_pressed,
            ctrl: false,
            shift: c >= b'A' && c <= b'Z',
        })
    }
}
