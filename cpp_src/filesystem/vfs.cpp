#include "filesystem/vfs.hpp"
#include <cstring>

VirtualFileSystem::VirtualFileSystem() {
    for (size_t i = 0; i < MAX_FILES; i++) {
        files[i].in_use = false;
        files[i].size = 0;
    }
}

void VirtualFileSystem::init() {
    // Already initialized in constructor
}

int VirtualFileSystem::find_inode_by_name(const char* name) {
    for (size_t i = 0; i < MAX_FILES; i++) {
        if (files[i].in_use) {
            bool match = true;
            for (size_t j = 0; j < MAX_FILENAME_LEN; j++) {
                if (files[i].name[j] != name[j]) {
                    match = false;
                    break;
                }
                if (name[j] == '\0') break;
            }
            if (match) return i;
        }
    }
    return -1;
}

int VirtualFileSystem::create_file(const char* name) {
    // Check if file already exists
    if (find_inode_by_name(name) >= 0) {
        return -1; // Already exists
    }
    
    // Find free slot
    for (size_t i = 0; i < MAX_FILES; i++) {
        if (!files[i].in_use) {
            files[i].in_use = true;
            files[i].size = 0;
            // Copy name
            for (size_t j = 0; j < MAX_FILENAME_LEN; j++) {
                files[i].name[j] = name[j];
                if (name[j] == '\0') break;
            }
            return i;
        }
    }
    return -1; // No space
}

bool VirtualFileSystem::delete_file(const char* name) {
    int inode = find_inode_by_name(name);
    if (inode >= 0) {
        files[inode].in_use = false;
        files[inode].size = 0;
        return true;
    }
    return false;
}

size_t VirtualFileSystem::read_file(int inode_id, uint8_t* buf, size_t buf_size) {
    if (inode_id < 0 || static_cast<size_t>(inode_id) >= MAX_FILES || !files[inode_id].in_use) {
        return 0;
    }
    
    size_t to_read = files[inode_id].size;
    if (to_read > buf_size) {
        to_read = buf_size;
    }
    
    for (size_t i = 0; i < to_read; i++) {
        buf[i] = files[inode_id].data[i];
    }
    
    return to_read;
}

bool VirtualFileSystem::write_file(int inode_id, const uint8_t* data, size_t size) {
    if (inode_id < 0 || static_cast<size_t>(inode_id) >= MAX_FILES || !files[inode_id].in_use) {
        return false;
    }
    
    if (size > MAX_FILE_SIZE) {
        size = MAX_FILE_SIZE;
    }
    
    for (size_t i = 0; i < size; i++) {
        files[inode_id].data[i] = data[i];
    }
    files[inode_id].size = size;
    
    return true;
}

size_t VirtualFileSystem::list_files(char file_list[][MAX_FILENAME_LEN], size_t max_files) {
    size_t count = 0;
    for (size_t i = 0; i < MAX_FILES && count < max_files; i++) {
        if (files[i].in_use) {
            for (size_t j = 0; j < MAX_FILENAME_LEN; j++) {
                file_list[count][j] = files[i].name[j];
                if (files[i].name[j] == '\0') break;
            }
            count++;
        }
    }
    return count;
}
