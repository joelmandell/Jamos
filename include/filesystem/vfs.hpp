#ifndef FILESYSTEM_VFS_HPP
#define FILESYSTEM_VFS_HPP

#include <cstddef>
#include <cstdint>

constexpr size_t MAX_FILES = 16;
constexpr size_t MAX_FILE_SIZE = 2048;
constexpr size_t MAX_FILENAME_LEN = 32;

struct FileEntry {
    char name[MAX_FILENAME_LEN];
    uint8_t data[MAX_FILE_SIZE];
    size_t size;
    bool in_use;
};

class VirtualFileSystem {
private:
    FileEntry files[MAX_FILES];
    
public:
    VirtualFileSystem();
    
    void init();
    int find_inode_by_name(const char* name);
    int create_file(const char* name);
    bool delete_file(const char* name);
    size_t read_file(int inode_id, uint8_t* buf, size_t buf_size);
    bool write_file(int inode_id, const uint8_t* data, size_t size);
    size_t list_files(char file_list[][MAX_FILENAME_LEN], size_t max_files);
};

#endif // FILESYSTEM_VFS_HPP
