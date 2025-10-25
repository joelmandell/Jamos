#include "main.hpp"
#include "utils.hpp"
#include <cstddef>

// Global static storage
VirtualDesktopManager g_vdm;
VirtualFileSystem g_vfs;
TextEditor g_editor;
WaylandCompositor g_wayland;

// Boot assembly stub
__asm__(
    ".section .text.boot\n"
    ".global _start\n"
    "\n"
    "_start:\n"
    "    // Set stack pointer to the end of the stack\n"
    "    ldr x30, =__stack_end\n"
    "    mov sp, x30\n"
    "    \n"
    "    // Clear BSS section\n"
    "    ldr x0, =__bss_start\n"
    "    ldr x1, =__bss_end\n"
    "clear_bss:\n"
    "    cmp x0, x1\n"
    "    b.ge clear_bss_done\n"
    "    str xzr, [x0], #8\n"
    "    b clear_bss\n"
    "clear_bss_done:\n"
    "    \n"
    "    // Branch to C++ main\n"
    "    bl rust_main\n"
    "    \n"
    "    // In case rust_main returns, loop forever\n"
    "hang:\n"
    "    wfe\n"
    "    b hang\n"
);

// Main entry point called from assembly
extern "C" [[noreturn]] void rust_main() {
    Uart uart;
    Keyboard keyboard;
    
    // Print initial message
    uart.puts("Hello lovely Anna!\n\n");
    uart.puts("=== Jamos Experimental Terminal ===\n");
    uart.puts("Ctrl+Right: New desktop | Ctrl+Left: Prev desktop | Ctrl+N: Name\n\n");
    
    // Initialize subsystems
    g_vdm.init();
    g_vfs.init();
    g_wayland.init();
    
    TerminalMode mode = TerminalMode::Normal;
    
    // Show prompt
    VirtualDesktop* desktop = g_vdm.current_mut();
    if (desktop) {
        desktop->screen_mut()->puts("[Desktop 1]$ ");
    }
    
    // Main terminal loop
    while (true) {
        KeyEvent event;
        if (keyboard.poll(event)) {
            switch (mode) {
                case TerminalMode::Normal:
                    handle_normal_mode(&g_vdm, event, mode);
                    break;
                case TerminalMode::NamingDesktop:
                    handle_naming_mode(&g_vdm, event, mode);
                    break;
                case TerminalMode::Editor:
                    handle_editor_mode(&g_vdm, event, mode);
                    break;
            }
        }
        
        // Small delay to avoid busy-waiting
        for (int i = 0; i < 1000; i++) {
            __asm__ volatile("nop");
        }
    }
}

void show_prompt(Screen* screen, const char* desktop_name) {
    screen->puts("[");
    screen->puts(desktop_name);
    screen->puts("]$ ");
}

void handle_normal_mode(VirtualDesktopManager* vdm, const KeyEvent& event, TerminalMode& mode) {
    // Handle Ctrl+Right: Create new virtual desktop
    if (event.ctrl && event.key == Key::Right) {
        if (vdm->next()) {
            VirtualDesktop* desktop = vdm->current_mut();
            if (desktop) {
                uint8_t name_buf[32];
                size_t name_len = desktop->copy_name_to(name_buf, 32);
                desktop->screen_mut()->clear();
                desktop->screen_mut()->puts(">>> Switched to ");
                for (size_t i = 0; i < name_len; i++) {
                    desktop->screen_mut()->putc(name_buf[i]);
                }
                desktop->screen_mut()->puts(" <<<\n\n");
                char name_str[33] = {0};
                copy_str(name_str, name_buf, name_len, 32);
                show_prompt(desktop->screen_mut(), name_str);
            }
        }
        return;
    }

    // Handle Ctrl+Left: Switch to previous desktop
    if (event.ctrl && event.key == Key::Left) {
        if (vdm->previous()) {
            VirtualDesktop* desktop = vdm->current_mut();
            if (desktop) {
                uint8_t name_buf[32];
                size_t name_len = desktop->copy_name_to(name_buf, 32);
                desktop->screen_mut()->puts("\n>>> Switched to ");
                for (size_t i = 0; i < name_len; i++) {
                    desktop->screen_mut()->putc(name_buf[i]);
                }
                desktop->screen_mut()->puts(" <<<\n");
                char name_str[33] = {0};
                copy_str(name_str, name_buf, name_len, 32);
                show_prompt(desktop->screen_mut(), name_str);
            }
        }
        return;
    }

    // Handle Ctrl+N: Name current desktop
    if (event.ctrl && event.key == Key::Char && 
        (event.char_value == 'n' || event.char_value == 'N')) {
        mode = TerminalMode::NamingDesktop;
        VirtualDesktop* desktop = vdm->current_mut();
        if (desktop) {
            desktop->clear_input();
            desktop->screen_mut()->puts("\n[Enter new name for desktop]: ");
        }
        return;
    }

    // Handle normal input
    VirtualDesktop* desktop = vdm->current_mut();
    if (!desktop) return;
    
    if (event.key == Key::Char) {
        desktop->screen_mut()->putc(event.char_value);
        desktop->add_input(event.char_value);
    } else if (event.key == Key::Enter) {
        size_t index = vdm->get_index();
        size_t count = vdm->get_count();
        
        const uint8_t* input = desktop->get_input();
        size_t input_len = desktop->get_input_len();
        
        desktop->screen_mut()->puts("\n");
        
        // Simple command handling
        if (str_eq(input, input_len, "help")) {
            desktop->screen_mut()->puts("Available commands:\n");
            desktop->screen_mut()->puts("  help    - Show this help\n");
            desktop->screen_mut()->puts("  clear   - Clear screen\n");
            desktop->screen_mut()->puts("  info    - Show desktop info\n");
            desktop->screen_mut()->puts("  ls      - List files\n");
            desktop->screen_mut()->puts("  touch   - Create file (usage: touch <name>)\n");
            desktop->screen_mut()->puts("  rm      - Delete file (usage: rm <name>)\n");
            desktop->screen_mut()->puts("  edit    - Edit file (usage: edit <name>)\n");
            desktop->screen_mut()->puts("  cat     - Display file (usage: cat <name>)\n");
            desktop->screen_mut()->puts("  wayland - Wayland compositor (usage: wayland [start|stop|status])\n");
        } else if (str_eq(input, input_len, "clear")) {
            desktop->screen_mut()->clear();
        } else if (str_eq(input, input_len, "info")) {
            uint8_t name_buf[32];
            size_t name_len = desktop->copy_name_to(name_buf, 32);
            desktop->screen_mut()->puts("Desktop: ");
            for (size_t i = 0; i < name_len; i++) {
                desktop->screen_mut()->putc(name_buf[i]);
            }
            desktop->screen_mut()->puts("\n");
            desktop->screen_mut()->puts("Index: ");
            print_number(desktop->screen_mut(), index + 1);
            desktop->screen_mut()->puts(" of ");
            print_number(desktop->screen_mut(), count);
            desktop->screen_mut()->puts("\n");
        } else if (str_eq(input, input_len, "ls")) {
            handle_ls_command(desktop->screen_mut());
        } else if (str_starts_with(input, input_len, "touch ")) {
            handle_touch_command(desktop->screen_mut(), input + 6, input_len - 6);
        } else if (str_starts_with(input, input_len, "rm ")) {
            handle_rm_command(desktop->screen_mut(), input + 3, input_len - 3);
        } else if (str_starts_with(input, input_len, "edit ")) {
            handle_edit_command(desktop->screen_mut(), input + 5, input_len - 5, mode);
            return;
        } else if (str_starts_with(input, input_len, "cat ")) {
            handle_cat_command(desktop->screen_mut(), input + 4, input_len - 4);
        } else if (str_eq(input, input_len, "wayland") || str_starts_with(input, input_len, "wayland ")) {
            handle_wayland_command(desktop->screen_mut(), input, input_len);
        } else if (input_len > 0) {
            desktop->screen_mut()->puts("Unknown command: ");
            for (size_t i = 0; i < input_len; i++) {
                desktop->screen_mut()->putc(input[i]);
            }
            desktop->screen_mut()->puts("\n");
        }
        
        desktop->clear_input();
        uint8_t name_buf[32];
        size_t name_len = desktop->copy_name_to(name_buf, 32);
        char name_str[33] = {0};
        copy_str(name_str, name_buf, name_len, 32);
        show_prompt(desktop->screen_mut(), name_str);
    } else if (event.key == Key::Backspace) {
        desktop->remove_input();
        desktop->screen_mut()->puts("\x08 \x08");
    }
}

void handle_naming_mode(VirtualDesktopManager* vdm, const KeyEvent& event, TerminalMode& mode) {
    VirtualDesktop* desktop = vdm->current_mut();
    if (!desktop) return;
    
    if (event.key == Key::Char) {
        desktop->screen_mut()->putc(event.char_value);
        desktop->add_input(event.char_value);
    } else if (event.key == Key::Enter) {
        const uint8_t* input = desktop->get_input();
        size_t input_len = desktop->get_input_len();
        
        if (input_len > 0) {
            char name_str[33] = {0};
            copy_str(name_str, input, input_len, 32);
            desktop->set_name(name_str);
            uint8_t name_buf[32];
            size_t name_len = desktop->copy_name_to(name_buf, 32);
            desktop->screen_mut()->puts("\n[Desktop renamed to: ");
            for (size_t i = 0; i < name_len; i++) {
                desktop->screen_mut()->putc(name_buf[i]);
            }
            desktop->screen_mut()->puts("]\n");
        } else {
            desktop->screen_mut()->puts("\n[Name unchanged]\n");
        }
        desktop->clear_input();
        uint8_t name_buf[32];
        size_t name_len = desktop->copy_name_to(name_buf, 32);
        char name_str[33] = {0};
        copy_str(name_str, name_buf, name_len, 32);
        show_prompt(desktop->screen_mut(), name_str);
        mode = TerminalMode::Normal;
    } else if (event.key == Key::Backspace) {
        desktop->remove_input();
        desktop->screen_mut()->puts("\x08 \x08");
    } else if (event.key == Key::Escape) {
        desktop->screen_mut()->puts("\n[Cancelled]\n");
        desktop->clear_input();
        uint8_t name_buf[32];
        size_t name_len = desktop->copy_name_to(name_buf, 32);
        char name_str[33] = {0};
        copy_str(name_str, name_buf, name_len, 32);
        show_prompt(desktop->screen_mut(), name_str);
        mode = TerminalMode::Normal;
    }
}

void handle_editor_mode(VirtualDesktopManager* vdm, const KeyEvent& event, TerminalMode& mode) {
    VirtualDesktop* desktop = vdm->current_mut();
    if (!desktop) return;
    
    EditorAction action = g_editor.handle_key(event, desktop->screen_mut());
    
    switch (action) {
        case EditorAction::Save:
        case EditorAction::SaveAndQuit: {
            const char* filename = g_editor.get_filename();
            const uint8_t* content = g_editor.get_content();
            size_t content_size = g_editor.get_content_size();
            
            int inode_id = g_vfs.find_inode_by_name(filename);
            if (inode_id < 0) {
                inode_id = g_vfs.create_file(filename);
                if (inode_id < 0) {
                    desktop->screen_mut()->puts("\nError creating file\n");
                    return;
                }
            }
            
            if (g_vfs.write_file(inode_id, content, content_size)) {
                g_editor.mark_saved();
                desktop->screen_mut()->puts("\nFile saved: ");
                desktop->screen_mut()->puts(filename);
                desktop->screen_mut()->puts("\n");
            } else {
                desktop->screen_mut()->puts("\nError saving file\n");
            }
            
            if (action == EditorAction::SaveAndQuit) {
                mode = TerminalMode::Normal;
                uint8_t name_buf[32];
                size_t name_len = desktop->copy_name_to(name_buf, 32);
                char name_str[33] = {0};
                copy_str(name_str, name_buf, name_len, 32);
                show_prompt(desktop->screen_mut(), name_str);
            }
            break;
        }
        case EditorAction::Quit: {
            mode = TerminalMode::Normal;
            desktop->screen_mut()->puts("\nEditor closed.\n");
            uint8_t name_buf[32];
            size_t name_len = desktop->copy_name_to(name_buf, 32);
            char name_str[33] = {0};
            copy_str(name_str, name_buf, name_len, 32);
            show_prompt(desktop->screen_mut(), name_str);
            break;
        }
        case EditorAction::Continue:
            // Continue editing
            break;
    }
}

// Command handlers
void handle_ls_command(Screen* screen) {
    char file_list[32][MAX_FILENAME_LEN];
    size_t count = g_vfs.list_files(file_list, 32);
    
    if (count == 0) {
        screen->puts("No files.\n");
    } else {
        screen->puts("Files:\n");
        for (size_t i = 0; i < count; i++) {
            screen->puts("  ");
            screen->puts(file_list[i]);
            screen->puts("\n");
        }
    }
}

void handle_touch_command(Screen* screen, const uint8_t* filename, size_t len) {
    if (len == 0) {
        screen->puts("Usage: touch <filename>\n");
        return;
    }
    
    char filename_str[MAX_FILENAME_LEN];
    copy_str(filename_str, filename, len, MAX_FILENAME_LEN - 1);
    
    int result = g_vfs.create_file(filename_str);
    if (result >= 0) {
        screen->puts("File created: ");
        screen->puts(filename_str);
        screen->puts("\n");
    } else {
        screen->puts("Error: Could not create file\n");
    }
}

void handle_rm_command(Screen* screen, const uint8_t* filename, size_t len) {
    if (len == 0) {
        screen->puts("Usage: rm <filename>\n");
        return;
    }
    
    char filename_str[MAX_FILENAME_LEN];
    copy_str(filename_str, filename, len, MAX_FILENAME_LEN - 1);
    
    if (g_vfs.delete_file(filename_str)) {
        screen->puts("File deleted: ");
        screen->puts(filename_str);
        screen->puts("\n");
    } else {
        screen->puts("Error: File not found\n");
    }
}

void handle_cat_command(Screen* screen, const uint8_t* filename, size_t len) {
    if (len == 0) {
        screen->puts("Usage: cat <filename>\n");
        return;
    }
    
    char filename_str[MAX_FILENAME_LEN];
    copy_str(filename_str, filename, len, MAX_FILENAME_LEN - 1);
    
    int inode_id = g_vfs.find_inode_by_name(filename_str);
    if (inode_id >= 0) {
        uint8_t buf[512];
        size_t size = g_vfs.read_file(inode_id, buf, 512);
        for (size_t i = 0; i < size; i++) {
            screen->putc(buf[i]);
        }
        screen->puts("\n");
    } else {
        screen->puts("File not found: ");
        screen->puts(filename_str);
        screen->puts("\n");
    }
}

void handle_edit_command(Screen* screen, const uint8_t* filename, size_t len, TerminalMode& mode) {
    if (len == 0) {
        screen->puts("Usage: edit <filename>\n");
        return;
    }
    
    char filename_str[EDITOR_FILENAME_SIZE];
    copy_str(filename_str, filename, len, EDITOR_FILENAME_SIZE - 1);
    
    g_editor.set_filename(filename_str);
    
    // Try to load existing file
    int inode_id = g_vfs.find_inode_by_name(filename_str);
    if (inode_id >= 0) {
        uint8_t buf[EDITOR_BUFFER_SIZE];
        size_t size = g_vfs.read_file(inode_id, buf, EDITOR_BUFFER_SIZE);
        g_editor.load_content(buf, size);
    }
    
    // Render editor
    g_editor.render(screen);
    mode = TerminalMode::Editor;
}

void handle_wayland_command(Screen* screen, const uint8_t* input, size_t len) {
    if (str_eq(input, len, "wayland") || str_eq(input, len, "wayland status")) {
        g_wayland.status(screen);
    } else if (str_eq(input, len, "wayland start")) {
        g_wayland.start(screen);
    } else if (str_eq(input, len, "wayland stop")) {
        g_wayland.stop(screen);
    } else {
        screen->puts("Usage: wayland [start|stop|status]\n");
        screen->puts("  start  - Start the Wayland compositor\n");
        screen->puts("  stop   - Stop the Wayland compositor\n");
        screen->puts("  status - Show compositor status (default)\n");
    }
}

// String utility functions
bool str_eq(const uint8_t* a, size_t a_len, const char* b) {
    size_t b_len = 0;
    while (b[b_len] != '\0') b_len++;
    
    if (a_len != b_len) return false;
    
    for (size_t i = 0; i < a_len; i++) {
        if (a[i] != static_cast<uint8_t>(b[i])) return false;
    }
    return true;
}

bool str_starts_with(const uint8_t* str, size_t str_len, const char* prefix) {
    size_t prefix_len = 0;
    while (prefix[prefix_len] != '\0') prefix_len++;
    
    if (str_len < prefix_len) return false;
    
    for (size_t i = 0; i < prefix_len; i++) {
        if (str[i] != static_cast<uint8_t>(prefix[i])) return false;
    }
    return true;
}

void copy_str(char* dest, const uint8_t* src, size_t len, size_t max_len) {
    size_t copy_len = (len < max_len) ? len : max_len;
    for (size_t i = 0; i < copy_len; i++) {
        dest[i] = static_cast<char>(src[i]);
    }
    dest[copy_len] = '\0';
}

// Panic handler
extern "C" void __cxa_pure_virtual() {
    Uart uart;
    uart.puts("\n\n*** PURE VIRTUAL FUNCTION CALLED ***\n");
    uart.puts("System halted.\n");
    while (true) {
        __asm__ volatile("wfe");
    }
}

// Operators new and delete (no-op implementations for bare metal)
// Note: These return nullptr which is acceptable for bare metal with -fno-exceptions
// Declarations without noexcept to match standard library expectations
void* operator new(std::size_t) { return nullptr; }
void* operator new[](std::size_t) { return nullptr; }
void operator delete(void*) noexcept {}
void operator delete[](void*) noexcept {}
void operator delete(void*, std::size_t) noexcept {}
void operator delete[](void*, std::size_t) noexcept {}
