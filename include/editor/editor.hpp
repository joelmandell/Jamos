#ifndef EDITOR_HPP
#define EDITOR_HPP

#include "terminal/screen.hpp"
#include "drivers/keyboard.hpp"
#include <cstddef>

constexpr size_t EDITOR_BUFFER_SIZE = 2048;
constexpr size_t EDITOR_FILENAME_SIZE = 32;

enum class EditorAction {
    Continue,
    Save,
    SaveAndQuit,
    Quit
};

class TextEditor {
private:
    uint8_t buffer[EDITOR_BUFFER_SIZE];
    size_t buffer_len;
    char filename[EDITOR_FILENAME_SIZE];
    size_t cursor;
    bool modified;
    
public:
    TextEditor() : buffer_len(0), cursor(0), modified(false) {
        filename[0] = '\0';
    }
    
    void set_filename(const char* name);
    const char* get_filename() const { return filename; }
    void load_content(const uint8_t* content, size_t size);
    const uint8_t* get_content() const { return buffer; }
    size_t get_content_size() const { return buffer_len; }
    void mark_saved() { modified = false; }
    void render(Screen* screen);
    EditorAction handle_key(const KeyEvent& event, Screen* screen);
};

#endif // EDITOR_HPP
