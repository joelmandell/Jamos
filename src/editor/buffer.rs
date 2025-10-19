// Simple nano-like text editor
use crate::terminal::Screen;
use crate::drivers::keyboard::{KeyEvent, Key};

const MAX_BUFFER_SIZE: usize = 2048;
const MAX_LINES: usize = 50;

pub struct TextEditor {
    buffer: [u8; MAX_BUFFER_SIZE],
    buffer_len: usize,
    cursor_pos: usize,
    filename: [u8; 32],
    filename_len: usize,
    modified: bool,
}

impl TextEditor {
    pub const fn empty() -> Self {
        TextEditor {
            buffer: [0; MAX_BUFFER_SIZE],
            buffer_len: 0,
            cursor_pos: 0,
            filename: [0; 32],
            filename_len: 0,
            modified: false,
        }
    }

    pub fn new() -> Self {
        TextEditor::empty()
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename_len = name.len().min(32);
        self.filename[..self.filename_len].copy_from_slice(&name.as_bytes()[..self.filename_len]);
    }

    pub fn get_filename(&self) -> &str {
        core::str::from_utf8(&self.filename[..self.filename_len]).unwrap_or("")
    }

    pub fn load_content(&mut self, data: &[u8]) {
        let len = data.len().min(MAX_BUFFER_SIZE);
        self.buffer[..len].copy_from_slice(&data[..len]);
        self.buffer_len = len;
        self.cursor_pos = len;
        self.modified = false;
    }

    pub fn get_content(&self) -> &[u8] {
        &self.buffer[..self.buffer_len]
    }

    pub fn insert_char(&mut self, c: u8) {
        if self.buffer_len < MAX_BUFFER_SIZE && self.cursor_pos <= self.buffer_len {
            // Shift characters to the right
            for i in (self.cursor_pos..self.buffer_len).rev() {
                self.buffer[i + 1] = self.buffer[i];
            }
            self.buffer[self.cursor_pos] = c;
            self.buffer_len += 1;
            self.cursor_pos += 1;
            self.modified = true;
        }
    }

    pub fn delete_char(&mut self) -> bool {
        if self.cursor_pos > 0 && self.buffer_len > 0 {
            // Shift characters to the left
            for i in self.cursor_pos..self.buffer_len {
                self.buffer[i - 1] = self.buffer[i];
            }
            self.buffer_len -= 1;
            self.cursor_pos -= 1;
            self.modified = true;
            true
        } else {
            false
        }
    }

    pub fn move_cursor_left(&mut self) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            true
        } else {
            false
        }
    }

    pub fn move_cursor_right(&mut self) -> bool {
        if self.cursor_pos < self.buffer_len {
            self.cursor_pos += 1;
            true
        } else {
            false
        }
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    pub fn render(&self, screen: &mut Screen) {
        screen.clear();
        screen.puts("=== Jamos Text Editor ===\n");
        screen.puts("File: ");
        screen.puts(self.get_filename());
        if self.modified {
            screen.puts(" [Modified]");
        }
        screen.puts("\n");
        screen.puts("Ctrl+S: Save | Ctrl+Q: Quit | Ctrl+X: Exit and Save\n");
        screen.puts("---\n");

        // Display buffer content
        for &byte in &self.buffer[..self.buffer_len] {
            screen.putc(byte);
        }
        
        screen.puts("\n---\n");
    }

    pub fn handle_key(&mut self, event: &KeyEvent, screen: &mut Screen) -> EditorAction {
        // Handle Ctrl+S: Save
        if event.ctrl && (event.key == Key::Char(b's') || event.key == Key::Char(b'S')) {
            return EditorAction::Save;
        }

        // Handle Ctrl+Q: Quit without saving
        if event.ctrl && (event.key == Key::Char(b'q') || event.key == Key::Char(b'Q')) {
            return EditorAction::Quit;
        }

        // Handle Ctrl+X: Save and quit
        if event.ctrl && (event.key == Key::Char(b'x') || event.key == Key::Char(b'X')) {
            return EditorAction::SaveAndQuit;
        }

        // Handle navigation
        match event.key {
            Key::Left => {
                self.move_cursor_left();
                self.render(screen);
            }
            Key::Right => {
                self.move_cursor_right();
                self.render(screen);
            }
            Key::Backspace => {
                if self.delete_char() {
                    self.render(screen);
                }
            }
            Key::Char(c) => {
                self.insert_char(c);
                self.render(screen);
            }
            Key::Enter => {
                self.insert_char(b'\n');
                self.render(screen);
            }
            _ => {}
        }

        EditorAction::Continue
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EditorAction {
    Continue,
    Save,
    Quit,
    SaveAndQuit,
}
