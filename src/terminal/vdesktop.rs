// Virtual desktop management with tiling terminal support
use super::screen::Screen;
use crate::drivers::uart::Uart;

const MAX_NAME_LEN: usize = 16;

pub struct VirtualDesktop {
    name: [u8; MAX_NAME_LEN],
    name_len: usize,
    screen: Screen,
    input_buffer: [u8; 32],
    input_len: usize,
    is_active: bool,
}

impl VirtualDesktop {
    pub const fn empty() -> Self {
        VirtualDesktop {
            name: [0; MAX_NAME_LEN],
            name_len: 0,
            screen: Screen::empty(),
            input_buffer: [0; 32],
            input_len: 0,
            is_active: false,
        }
    }
    
    pub fn init(&mut self, uart: Uart, name: &str) {
        self.screen = Screen::new(uart);
        self.set_name(name);
        self.is_active = true;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name_len = name.len().min(MAX_NAME_LEN);
        self.name[..self.name_len].copy_from_slice(&name.as_bytes()[..self.name_len]);
    }

    pub fn copy_name_to(&self, buf: &mut [u8]) -> usize {
        let len = self.name_len.min(buf.len());
        buf[..len].copy_from_slice(&self.name[..len]);
        len
    }

    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    pub fn add_input(&mut self, c: u8) {
        if self.input_len < self.input_buffer.len() {
            self.input_buffer[self.input_len] = c;
            self.input_len += 1;
        }
    }

    pub fn remove_input(&mut self) {
        if self.input_len > 0 {
            self.input_len -= 1;
        }
    }

    pub fn get_input(&self) -> &[u8] {
        &self.input_buffer[..self.input_len]
    }

    pub fn clear_input(&mut self) {
        self.input_len = 0;
    }
}

pub struct VirtualDesktopManager {
    desktop1: VirtualDesktop,
    desktop2: VirtualDesktop,
    current_index: usize,
}

impl VirtualDesktopManager {
    pub const fn empty() -> Self {
        VirtualDesktopManager {
            desktop1: VirtualDesktop::empty(),
            desktop2: VirtualDesktop::empty(),
            current_index: 0,
        }
    }
    
    pub fn init(&mut self, uart: Uart) {
        self.desktop1.init(uart, "Desktop 1");
    }

    pub fn current_mut(&mut self) -> Option<&mut VirtualDesktop> {
        match self.current_index {
            0 if self.desktop1.is_active => Some(&mut self.desktop1),
            1 if self.desktop2.is_active => Some(&mut self.desktop2),
            _ => None,
        }
    }

    pub fn next(&mut self, uart: Uart) -> bool {
        if !self.desktop2.is_active && self.current_index == 0 {
            // Activate second desktop
            self.desktop2.init(uart, "Desktop 2");
            self.current_index = 1;
            true
        } else if self.current_index == 0 && self.desktop2.is_active {
            // Switch to second desktop
            self.current_index = 1;
            true
        } else {
            false
        }
    }

    pub fn previous(&mut self) -> bool {
        if self.current_index > 0 {
            self.current_index -= 1;
            true
        } else {
            false
        }
    }

    pub fn get_index(&self) -> usize {
        self.current_index
    }

    pub fn get_count(&self) -> usize {
        if self.desktop2.is_active { 2 } else { 1 }
    }
}
