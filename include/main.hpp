#ifndef MAIN_HPP
#define MAIN_HPP

#include "drivers/uart.hpp"
#include "drivers/keyboard.hpp"
#include "terminal/screen.hpp"
#include "terminal/vdesktop.hpp"
#include "filesystem/vfs.hpp"
#include "editor/editor.hpp"
#include "wayland/compositor.hpp"

enum class TerminalMode {
    Normal,
    NamingDesktop,
    Editor
};

// Global storage
extern VirtualDesktopManager g_vdm;
extern VirtualFileSystem g_vfs;
extern TextEditor g_editor;
extern WaylandCompositor g_wayland;

// Boot entry point
extern "C" [[noreturn]] void rust_main();

// Helper functions
void show_prompt(Screen* screen, const char* desktop_name);
void handle_normal_mode(VirtualDesktopManager* vdm, const KeyEvent& event, TerminalMode& mode);
void handle_naming_mode(VirtualDesktopManager* vdm, const KeyEvent& event, TerminalMode& mode);
void handle_editor_mode(VirtualDesktopManager* vdm, const KeyEvent& event, TerminalMode& mode);

// Command handlers
void handle_ls_command(Screen* screen);
void handle_touch_command(Screen* screen, const uint8_t* filename, size_t len);
void handle_rm_command(Screen* screen, const uint8_t* filename, size_t len);
void handle_cat_command(Screen* screen, const uint8_t* filename, size_t len);
void handle_edit_command(Screen* screen, const uint8_t* filename, size_t len, TerminalMode& mode);
void handle_wayland_command(Screen* screen, const uint8_t* input, size_t len);

// String utility functions
bool str_eq(const uint8_t* a, size_t a_len, const char* b);
bool str_starts_with(const uint8_t* str, size_t str_len, const char* prefix);
void copy_str(char* dest, const uint8_t* src, size_t len, size_t max_len);

#endif // MAIN_HPP
