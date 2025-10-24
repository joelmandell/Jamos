#include "editor/editor.hpp"

void TextEditor::set_filename(const char* name) {
    size_t i = 0;
    while (name[i] != '\0' && i < EDITOR_FILENAME_SIZE - 1) {
        filename[i] = name[i];
        i++;
    }
    filename[i] = '\0';
}

void TextEditor::load_content(const uint8_t* content, size_t size) {
    if (size > EDITOR_BUFFER_SIZE) {
        size = EDITOR_BUFFER_SIZE;
    }
    for (size_t i = 0; i < size; i++) {
        buffer[i] = content[i];
    }
    buffer_len = size;
    cursor = size;
    modified = false;
}

void TextEditor::render(Screen* screen) {
    screen->clear();
    screen->puts("=== Editor: ");
    screen->puts(filename);
    screen->puts(" ===\n");
    screen->puts("Ctrl+S: Save | Ctrl+X: Save & Quit | Ctrl+Q: Quit\n");
    screen->puts("---\n");
    
    for (size_t i = 0; i < buffer_len; i++) {
        screen->putc(buffer[i]);
    }
    screen->puts("\n---\n");
}

EditorAction TextEditor::handle_key(const KeyEvent& event, Screen* screen) {
    // Handle Ctrl commands
    if (event.ctrl && event.key == Key::Char) {
        if (event.char_value == 's' || event.char_value == 'S') {
            return EditorAction::Save;
        } else if (event.char_value == 'x' || event.char_value == 'X') {
            return EditorAction::SaveAndQuit;
        } else if (event.char_value == 'q' || event.char_value == 'Q') {
            return EditorAction::Quit;
        }
    }
    
    // Handle regular input
    if (event.key == Key::Char) {
        if (buffer_len < EDITOR_BUFFER_SIZE) {
            buffer[buffer_len++] = event.char_value;
            screen->putc(event.char_value);
            modified = true;
        }
    } else if (event.key == Key::Backspace) {
        if (buffer_len > 0) {
            buffer_len--;
            screen->puts("\x08 \x08");
            modified = true;
        }
    } else if (event.key == Key::Enter) {
        if (buffer_len < EDITOR_BUFFER_SIZE) {
            buffer[buffer_len++] = '\n';
            screen->putc('\n');
            modified = true;
        }
    }
    
    return EditorAction::Continue;
}
