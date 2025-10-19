#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod drivers;
mod terminal;
mod filesystem;
mod editor;
mod wayland;
mod utils;

use drivers::{uart::Uart, keyboard::{Keyboard, Key, KeyEvent}};
use terminal::{VirtualDesktopManager, Screen};
use filesystem::VirtualFileSystem;
use editor::{TextEditor, buffer::EditorAction};
use wayland::WaylandCompositor;
use utils::print_number;

// Global static storage for the virtual desktop manager
static mut VDM_STORAGE: VirtualDesktopManager = VirtualDesktopManager::empty();
static mut VFS_STORAGE: VirtualFileSystem = VirtualFileSystem::empty();
static mut EDITOR_STORAGE: TextEditor = TextEditor::empty();
static mut WAYLAND_STORAGE: WaylandCompositor = WaylandCompositor::empty();

fn get_vdm() -> &'static mut VirtualDesktopManager {
    unsafe {
        &mut VDM_STORAGE
    }
}

fn get_vfs() -> &'static mut VirtualFileSystem {
    unsafe {
        &mut VFS_STORAGE
    }
}

fn get_editor() -> &'static mut TextEditor {
    unsafe {
        &mut EDITOR_STORAGE
    }
}

fn get_wayland() -> &'static mut WaylandCompositor {
    unsafe {
        &mut WAYLAND_STORAGE
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
    Editor,
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
    
    // Initialize virtual filesystem
    get_vfs().init();
    
    // Initialize Wayland compositor
    get_wayland().init();
    uart.puts("After Wayland init\n");
    
    let mut mode = TerminalMode::Normal;
    uart.puts("After mode init\n");
    
    // Show prompt
    uart.puts("A\n");
    {
        let vdm = get_vdm();
        uart.puts("B\n");
        if let Some(desktop) = vdm.current_mut() {
            uart.puts("C\n");
            desktop.screen_mut().puts("[Desktop 1]$ ");
            uart.puts("D\n");
        } else {
            // Fallback if desktop not initialized
            uart.puts("E\n");
            uart.puts("[Desktop 1]$ ");
        }
    }
    uart.puts("F\n");
    
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
                TerminalMode::Editor => {
                    handle_editor_mode(&event, &mut mode);
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
                    desktop.screen_mut().puts("  ls      - List files\n");
                    desktop.screen_mut().puts("  touch   - Create file (usage: touch <name>)\n");
                    desktop.screen_mut().puts("  rm      - Delete file (usage: rm <name>)\n");
                    desktop.screen_mut().puts("  edit    - Edit file (usage: edit <name>)\n");
                    desktop.screen_mut().puts("  cat     - Display file (usage: cat <name>)\n");
                    desktop.screen_mut().puts("  wayland - Wayland compositor (usage: wayland [start|stop|status])\n");
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
                } else if input == b"ls" {
                    handle_ls_command(desktop.screen_mut());
                } else if input.starts_with(b"touch ") {
                    let filename = &input[6..];
                    handle_touch_command(desktop.screen_mut(), filename);
                } else if input.starts_with(b"rm ") {
                    let filename = &input[3..];
                    handle_rm_command(desktop.screen_mut(), filename);
                } else if input.starts_with(b"edit ") {
                    let filename = &input[5..];
                    handle_edit_command(desktop.screen_mut(), filename, mode);
                    return;
                } else if input.starts_with(b"cat ") {
                    let filename = &input[4..];
                    handle_cat_command(desktop.screen_mut(), filename);
                } else if input == b"wayland" || input.starts_with(b"wayland ") {
                    handle_wayland_command(desktop.screen_mut(), input);
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



fn handle_ls_command(screen: &mut Screen) {
    let vfs = get_vfs();
    let mut file_list = [[0u8; 32]; 32];
    let count = vfs.list_files(&mut file_list);
    
    if count == 0 {
        screen.puts("No files.\n");
    } else {
        screen.puts("Files:\n");
        for i in 0..count {
            screen.puts("  ");
            let mut j = 0;
            while j < 32 && file_list[i][j] != 0 {
                screen.putc(file_list[i][j]);
                j += 1;
            }
            screen.puts("\n");
        }
    }
}

fn handle_touch_command(screen: &mut Screen, filename: &[u8]) {
    if filename.is_empty() {
        screen.puts("Usage: touch <filename>\n");
        return;
    }
    
    let vfs = get_vfs();
    let filename_str = core::str::from_utf8(filename).unwrap_or("");
    
    match vfs.create_file(filename_str) {
        Ok(_) => {
            screen.puts("File created: ");
            screen.puts(filename_str);
            screen.puts("\n");
        }
        Err(e) => {
            screen.puts("Error: ");
            screen.puts(e);
            screen.puts("\n");
        }
    }
}

fn handle_rm_command(screen: &mut Screen, filename: &[u8]) {
    if filename.is_empty() {
        screen.puts("Usage: rm <filename>\n");
        return;
    }
    
    let vfs = get_vfs();
    let filename_str = core::str::from_utf8(filename).unwrap_or("");
    
    match vfs.delete_file(filename_str) {
        Ok(_) => {
            screen.puts("File deleted: ");
            screen.puts(filename_str);
            screen.puts("\n");
        }
        Err(e) => {
            screen.puts("Error: ");
            screen.puts(e);
            screen.puts("\n");
        }
    }
}

fn handle_cat_command(screen: &mut Screen, filename: &[u8]) {
    if filename.is_empty() {
        screen.puts("Usage: cat <filename>\n");
        return;
    }
    
    let vfs = get_vfs();
    let filename_str = core::str::from_utf8(filename).unwrap_or("");
    
    match vfs.find_inode_by_name(filename_str) {
        Some(inode_id) => {
            let mut buf = [0u8; 512];
            match vfs.read_file(inode_id, &mut buf) {
                Ok(size) => {
                    for &byte in &buf[..size] {
                        screen.putc(byte);
                    }
                    screen.puts("\n");
                }
                Err(e) => {
                    screen.puts("Error reading file: ");
                    screen.puts(e);
                    screen.puts("\n");
                }
            }
        }
        None => {
            screen.puts("File not found: ");
            screen.puts(filename_str);
            screen.puts("\n");
        }
    }
}

fn handle_edit_command(screen: &mut Screen, filename: &[u8], mode: &mut TerminalMode) {
    if filename.is_empty() {
        screen.puts("Usage: edit <filename>\n");
        return;
    }
    
    let filename_str = core::str::from_utf8(filename).unwrap_or("");
    let editor = get_editor();
    editor.set_filename(filename_str);
    
    // Try to load existing file
    let vfs = get_vfs();
    if let Some(inode_id) = vfs.find_inode_by_name(filename_str) {
        let mut buf = [0u8; 2048];
        match vfs.read_file(inode_id, &mut buf) {
            Ok(size) => {
                editor.load_content(&buf[..size]);
            }
            Err(_) => {
                // File exists but can't read, start with empty buffer
            }
        }
    }
    
    // Render editor
    editor.render(screen);
    *mode = TerminalMode::Editor;
}

fn handle_wayland_command(screen: &mut Screen, input: &[u8]) {
    let wayland = get_wayland();
    
    if input == b"wayland" || input == b"wayland status" {
        wayland.status(screen);
    } else if input == b"wayland start" {
        wayland.start(screen);
    } else if input == b"wayland stop" {
        wayland.stop(screen);
    } else {
        screen.puts("Usage: wayland [start|stop|status]\n");
        screen.puts("  start  - Start the Wayland compositor\n");
        screen.puts("  stop   - Stop the Wayland compositor\n");
        screen.puts("  status - Show compositor status (default)\n");
    }
}

fn handle_editor_mode(event: &KeyEvent, mode: &mut TerminalMode) {
    let editor = get_editor();
    let vdm = get_vdm();
    
    if let Some(desktop) = vdm.current_mut() {
        let action = editor.handle_key(event, desktop.screen_mut());
        
        match action {
            EditorAction::Save | EditorAction::SaveAndQuit => {
                let mut filename_buf = [0u8; 32];
                let filename = editor.get_filename();
                let len = filename.len().min(32);
                filename_buf[..len].copy_from_slice(filename.as_bytes());
                let filename_str = core::str::from_utf8(&filename_buf[..len]).unwrap_or("");
                
                let content = editor.get_content();
                let vfs = get_vfs();
                
                // Create file if it doesn't exist
                let inode_id = match vfs.find_inode_by_name(filename_str) {
                    Some(id) => id,
                    None => {
                        match vfs.create_file(filename_str) {
                            Ok(id) => id,
                            Err(e) => {
                                desktop.screen_mut().puts("\nError creating file: ");
                                desktop.screen_mut().puts(e);
                                desktop.screen_mut().puts("\n");
                                return;
                            }
                        }
                    }
                };
                
                // Write content
                match vfs.write_file(inode_id, content) {
                    Ok(_) => {
                        editor.mark_saved();
                        desktop.screen_mut().puts("\nFile saved: ");
                        desktop.screen_mut().puts(filename_str);
                        desktop.screen_mut().puts("\n");
                    }
                    Err(e) => {
                        desktop.screen_mut().puts("\nError saving file: ");
                        desktop.screen_mut().puts(e);
                        desktop.screen_mut().puts("\n");
                    }
                }
                
                if action == EditorAction::SaveAndQuit {
                    *mode = TerminalMode::Normal;
                    let mut name_buf = [0u8; 32];
                    let name_len = desktop.copy_name_to(&mut name_buf);
                    let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                    show_prompt(desktop.screen_mut(), name);
                }
            }
            EditorAction::Quit => {
                *mode = TerminalMode::Normal;
                desktop.screen_mut().puts("\nEditor closed.\n");
                let mut name_buf = [0u8; 32];
                let name_len = desktop.copy_name_to(&mut name_buf);
                let name = core::str::from_utf8(&name_buf[..name_len]).unwrap_or("???");
                show_prompt(desktop.screen_mut(), name);
            }
            EditorAction::Continue => {
                // Continue editing
            }
        }
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
