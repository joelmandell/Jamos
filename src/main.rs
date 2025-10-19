#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod drivers;
mod terminal;

use drivers::{uart::Uart, keyboard::{Keyboard, Key, KeyEvent}};
use terminal::{VirtualDesktopManager, Screen};

// Global static storage for the virtual desktop manager
static mut VDM_STORAGE: VirtualDesktopManager = VirtualDesktopManager::empty();

fn get_vdm() -> &'static mut VirtualDesktopManager {
    unsafe {
        &mut VDM_STORAGE
    }
}

// Assembly boot stub to initialize stack pointer before calling Rust
global_asm!(
    r#"
    .section .text.boot
    .global _start
    
_start:
    // Set stack pointer to the end of the stack
    ldr x30, =__stack_end
    mov sp, x30
    
    // Clear BSS section
    ldr x0, =__bss_start
    ldr x1, =__bss_end
clear_bss:
    cmp x0, x1
    b.ge clear_bss_done
    str xzr, [x0], #8
    b clear_bss
clear_bss_done:
    
    // Branch to Rust main
    bl rust_main
    
    // In case rust_main returns, loop forever
hang:
    wfe
    b hang
    "#
);

enum TerminalMode {
    Normal,
    NamingDesktop,
}

// Assembly boot stub calls this function after setting up the stack
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let uart = Uart::new();
    let mut keyboard = Keyboard::new(Uart::new());
    
    // Print initial message
    uart.puts("Hello lovely Anna!\n\n");
    uart.puts("=== Jamos Experimental Terminal ===\n");
    uart.puts("Ctrl+Right: New desktop | Ctrl+Left: Prev desktop | Ctrl+N: Name\n\n");
    
    // Initialize virtual desktop manager in global storage
    get_vdm().init(Uart::new());
    
    let mut mode = TerminalMode::Normal;
    
    // Show prompt
    {
        let vdm = get_vdm();
        if let Some(desktop) = vdm.current_mut() {
            desktop.screen_mut().puts("[Desktop 1]$ ");
        }
    }
    
    // Main terminal loop
    loop {
        let vdm = get_vdm();
        if let Some(event) = keyboard.poll() {
            match mode {
                TerminalMode::Normal => {
                    handle_normal_mode(vdm, &event, &mut mode);
                }
                TerminalMode::NamingDesktop => {
                    handle_naming_mode(vdm, &event, &mut mode);
                }
            }
        }
        
        // Small delay to avoid busy-waiting
        for _ in 0..1000 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
}

fn handle_normal_mode(
    vdm: &mut VirtualDesktopManager,
    event: &KeyEvent,
    mode: &mut TerminalMode,
) {
    // Handle Ctrl+Right: Create new virtual desktop
    if event.ctrl && event.key == Key::Right {
        let uart = Uart::new();
        if vdm.next(uart) {
            if let Some(desktop) = vdm.current_mut() {
                let mut name_buf = [0u8; 32];
                let name_len = desktop.copy_name_to(&mut name_buf);
                let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                desktop.screen_mut().clear();
                desktop.screen_mut().puts(">>> Switched to ");
                desktop.screen_mut().puts(name);
                desktop.screen_mut().puts(" <<<\n\n");
                show_prompt(desktop.screen_mut(), name);
            }
        }
        return;
    }

    // Handle Ctrl+Left: Switch to previous desktop
    if event.ctrl && event.key == Key::Left {
        if vdm.previous() {
            if let Some(desktop) = vdm.current_mut() {
                let mut name_buf = [0u8; 32];
                let name_len = desktop.copy_name_to(&mut name_buf);
                let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                // Show which desktop we switched to
                desktop.screen_mut().puts("\n>>> Switched to ");
                desktop.screen_mut().puts(name);
                desktop.screen_mut().puts(" <<<\n");
                show_prompt(desktop.screen_mut(), name);
            }
        }
        return;
    }

    // Handle Ctrl+N: Name current desktop
    if event.ctrl && event.key == Key::Char(b'n') || event.key == Key::Char(b'N') {
        *mode = TerminalMode::NamingDesktop;
        if let Some(desktop) = vdm.current_mut() {
            desktop.clear_input();
            desktop.screen_mut().puts("\n[Enter new name for desktop]: ");
        }
        return;
    }

    // Handle normal input
    match event.key {
        Key::Char(c) => {
            if let Some(desktop) = vdm.current_mut() {
                desktop.screen_mut().putc(c);
                desktop.add_input(c);
            }
        }
        Key::Enter => {
            let index = vdm.get_index();
            let count = vdm.get_count();
            
            if let Some(desktop) = vdm.current_mut() {
                // Copy input to local buffer before processing
                let mut input_buf = [0u8; 256];
                let input_slice = desktop.get_input();
                let input_len = input_slice.len();
                input_buf[..input_len].copy_from_slice(input_slice);
                let input = &input_buf[..input_len];
                
                desktop.screen_mut().puts("\n");
                
                // Simple command handling
                if input == b"help" {
                    desktop.screen_mut().puts("Available commands:\n");
                    desktop.screen_mut().puts("  help    - Show this help\n");
                    desktop.screen_mut().puts("  clear   - Clear screen\n");
                    desktop.screen_mut().puts("  info    - Show desktop info\n");
                } else if input == b"clear" {
                    desktop.screen_mut().clear();
                } else if input == b"info" {
                    let mut name_buf = [0u8; 32];
                    let name_len = desktop.copy_name_to(&mut name_buf);
                    let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                    desktop.screen_mut().puts("Desktop: ");
                    desktop.screen_mut().puts(name);
                    desktop.screen_mut().puts("\n");
                    desktop.screen_mut().puts("Index: ");
                    print_number(desktop.screen_mut(), index + 1);
                    desktop.screen_mut().puts(" of ");
                    print_number(desktop.screen_mut(), count);
                    desktop.screen_mut().puts("\n");
                } else if input.len() > 0 {
                    desktop.screen_mut().puts("Unknown command: ");
                    for &b in input {
                        desktop.screen_mut().putc(b);
                    }
                    desktop.screen_mut().puts("\n");
                }
                
                desktop.clear_input();
                let mut name_buf = [0u8; 32];
                let name_len = desktop.copy_name_to(&mut name_buf);
                let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                show_prompt(desktop.screen_mut(), name);
            }
        }
        Key::Backspace => {
            if let Some(desktop) = vdm.current_mut() {
                desktop.remove_input();
                // Send backspace sequence: BS + space + BS
                desktop.screen_mut().puts("\x08 \x08");
            }
        }
        _ => {}
    }
}

fn handle_naming_mode(
    vdm: &mut VirtualDesktopManager,
    event: &KeyEvent,
    mode: &mut TerminalMode,
) {
    match event.key {
        Key::Char(c) => {
            if let Some(desktop) = vdm.current_mut() {
                desktop.screen_mut().putc(c);
                desktop.add_input(c);
            }
        }
        Key::Enter => {
            if let Some(desktop) = vdm.current_mut() {
                // Copy input to local buffer before processing
                let mut input_buf = [0u8; 256];
                let input_slice = desktop.get_input();
                let input_len = input_slice.len();
                input_buf[..input_len].copy_from_slice(input_slice);
                
                if input_len > 0 {
                    let name_str = core::str::from_utf8(&input_buf[..input_len]).unwrap_or("Unnamed");
                    desktop.set_name(name_str);
                    let mut name_buf = [0u8; 32];
                    let name_len = desktop.copy_name_to(&mut name_buf);
                    let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                    desktop.screen_mut().puts("\n[Desktop renamed to: ");
                    desktop.screen_mut().puts(name);
                    desktop.screen_mut().puts("]\n");
                } else {
                    desktop.screen_mut().puts("\n[Name unchanged]\n");
                }
                desktop.clear_input();
                let mut name_buf = [0u8; 32];
                let name_len = desktop.copy_name_to(&mut name_buf);
                let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                show_prompt(desktop.screen_mut(), name);
            }
            *mode = TerminalMode::Normal;
        }
        Key::Backspace => {
            if let Some(desktop) = vdm.current_mut() {
                desktop.remove_input();
                // Send backspace sequence: BS + space + BS
                desktop.screen_mut().puts("\x08 \x08");
            }
        }
        Key::Escape => {
            if let Some(desktop) = vdm.current_mut() {
                desktop.screen_mut().puts("\n[Cancelled]\n");
                desktop.clear_input();
                let mut name_buf = [0u8; 32];
                let name_len = desktop.copy_name_to(&mut name_buf);
                let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                show_prompt(desktop.screen_mut(), name);
            }
            *mode = TerminalMode::Normal;
        }
        _ => {}
    }
}

fn show_prompt(screen: &mut Screen, desktop_name: &str) {
    screen.puts("[");
    screen.puts(desktop_name);
    screen.puts("]$ ");
}

fn print_number(screen: &mut Screen, n: usize) {
    let mut buf = [0u8; 20];
    let mut num = n;
    let mut len = 0;
    
    if num == 0 {
        buf[0] = b'0';
        len = 1;
    } else {
        while num > 0 {
            buf[len] = b'0' + (num % 10) as u8;
            num /= 10;
            len += 1;
        }
    }
    
    // Print in reverse
    for i in (0..len).rev() {
        screen.putc(buf[i]);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let uart = Uart::new();
    uart.puts("\n\n*** PANIC ***\n");
    if let Some(location) = info.location() {
        uart.puts("Location: ");
        uart.puts(location.file());
        uart.puts(":");
        // Print line number
        let line = location.line();
        let mut buf = [0u8; 10];
        let mut num = line;
        let mut len = 0;
        while num > 0 {
            buf[len] = b'0' + (num % 10) as u8;
            num /= 10;
            len += 1;
        }
        for i in (0..len).rev() {
            uart.putc(buf[i]);
        }
        uart.puts("\n");
    }
    uart.puts("System halted.\n");
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}
