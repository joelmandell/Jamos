// Screen buffer and rendering - simplified version that writes directly to UART
use crate::drivers::uart::Uart;

#[derive(Clone, Copy)]
pub struct Screen {
    uart: Uart,
    prompt_shown: bool,
}

impl Screen {
    pub const fn empty() -> Self {
        Screen {
            uart: Uart::empty(),
            prompt_shown: false,
        }
    }
    
    pub fn new(uart: Uart) -> Self {
        Screen {
            uart,
            prompt_shown: false,
        }
    }

    pub fn clear(&mut self) {
        // Clear screen using ANSI escape codes
        self.uart.puts("\x1B[2J\x1B[H");
        self.prompt_shown = false;
    }

    pub fn putc(&mut self, c: u8) {
        self.uart.putc(c);
    }

    pub fn puts(&mut self, s: &str) {
        self.uart.puts(s);
    }

    pub fn render(&self) {
        // No-op for direct output mode
    }
}
