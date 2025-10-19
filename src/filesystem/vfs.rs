// Virtual File System with PostgreSQL-inspired metadata
use super::metadata::{Inode, FileType, Metadata};

const MAX_INODES: usize = 64;
const MAX_FILES: usize = 32;
const MAX_FILENAME_LEN: usize = 32;
const DATA_BLOCK_SIZE: usize = 512;
const MAX_DATA_BLOCKS: usize = 32;

#[derive(Clone, Copy)]
pub struct FileEntry {
    pub name: [u8; MAX_FILENAME_LEN],
    pub name_len: usize,
    pub inode_id: usize,
    pub is_valid: bool,
}

impl FileEntry {
    pub const fn empty() -> Self {
        FileEntry {
            name: [0; MAX_FILENAME_LEN],
            name_len: 0,
            inode_id: 0,
            is_valid: false,
        }
    }

    pub fn new(name: &str, inode_id: usize) -> Self {
        let mut entry = FileEntry::empty();
        entry.name_len = name.len().min(MAX_FILENAME_LEN);
        entry.name[..entry.name_len].copy_from_slice(&name.as_bytes()[..entry.name_len]);
        entry.inode_id = inode_id;
        entry.is_valid = true;
        entry
    }

    pub fn matches(&self, name: &str) -> bool {
        self.is_valid && self.name_len == name.len() && &self.name[..self.name_len] == name.as_bytes()
    }
}

pub struct FileHandle {
    pub inode_id: usize,
    pub offset: usize,
}

pub struct VirtualFileSystem {
    inodes: [Inode; MAX_INODES],
    files: [FileEntry; MAX_FILES],
    data_storage: [[u8; DATA_BLOCK_SIZE]; MAX_DATA_BLOCKS],
    next_inode_id: usize,
    timestamp_counter: u64,
}

impl VirtualFileSystem {
    pub const fn empty() -> Self {
        VirtualFileSystem {
            inodes: [Inode::empty(); MAX_INODES],
            files: [FileEntry::empty(); MAX_FILES],
            data_storage: [[0; DATA_BLOCK_SIZE]; MAX_DATA_BLOCKS],
            next_inode_id: 1,
            timestamp_counter: 0,
        }
    }

    pub fn init(&mut self) {
        // Create root directory (inode 0)
        self.inodes[0] = Inode::new(0, FileType::Directory, 0);
        self.inodes[0].metadata.created_at = self.get_timestamp();
        self.inodes[0].metadata.modified_at = self.inodes[0].metadata.created_at;
    }

    fn get_timestamp(&mut self) -> u64 {
        self.timestamp_counter += 1;
        self.timestamp_counter
    }

    fn allocate_inode(&mut self, file_type: FileType, parent_id: usize) -> Option<usize> {
        if self.next_inode_id >= MAX_INODES {
            return None;
        }
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        
        self.inodes[id] = Inode::new(id, file_type, parent_id);
        self.inodes[id].metadata.created_at = self.get_timestamp();
        self.inodes[id].metadata.modified_at = self.inodes[id].metadata.created_at;
        
        Some(id)
    }

    fn find_file_entry(&self, name: &str) -> Option<&FileEntry> {
        self.files.iter().find(|e| e.matches(name))
    }

    fn find_free_file_entry(&mut self) -> Option<usize> {
        self.files.iter().position(|e| !e.is_valid)
    }

    pub fn create_file(&mut self, name: &str) -> Result<usize, &'static str> {
        // Check if file already exists
        if self.find_file_entry(name).is_some() {
            return Err("File already exists");
        }

        // Allocate inode
        let inode_id = self.allocate_inode(FileType::Regular, 0)
            .ok_or("No more inodes available")?;

        // Find free file entry
        let entry_idx = self.find_free_file_entry()
            .ok_or("No more file entries available")?;

        // Create file entry
        self.files[entry_idx] = FileEntry::new(name, inode_id);

        Ok(inode_id)
    }

    pub fn write_file(&mut self, inode_id: usize, data: &[u8]) -> Result<usize, &'static str> {
        if inode_id >= MAX_INODES || !self.inodes[inode_id].is_valid {
            return Err("Invalid inode");
        }

        if self.inodes[inode_id].metadata.file_type != FileType::Regular {
            return Err("Not a regular file");
        }

        // Find or allocate data block
        let block_id = self.inodes[inode_id].data_offset;
        if block_id >= MAX_DATA_BLOCKS {
            return Err("No data blocks available");
        }

        let bytes_to_write = data.len().min(DATA_BLOCK_SIZE);
        self.data_storage[block_id][..bytes_to_write].copy_from_slice(&data[..bytes_to_write]);
        
        let timestamp = self.get_timestamp();
        self.inodes[inode_id].metadata.size = bytes_to_write;
        self.inodes[inode_id].metadata.modified_at = timestamp;

        Ok(bytes_to_write)
    }

    pub fn read_file(&self, inode_id: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        if inode_id >= MAX_INODES || !self.inodes[inode_id].is_valid {
            return Err("Invalid inode");
        }

        let inode = &self.inodes[inode_id];
        if inode.metadata.file_type != FileType::Regular {
            return Err("Not a regular file");
        }

        let block_id = inode.data_offset;
        if block_id >= MAX_DATA_BLOCKS {
            return Err("Invalid data block");
        }

        let bytes_to_read = inode.metadata.size.min(buf.len());
        buf[..bytes_to_read].copy_from_slice(&self.data_storage[block_id][..bytes_to_read]);

        Ok(bytes_to_read)
    }

    pub fn list_files(&self, buf: &mut [[u8; MAX_FILENAME_LEN]; MAX_FILES]) -> usize {
        let mut count = 0;
        for entry in &self.files {
            if entry.is_valid && count < MAX_FILES {
                buf[count][..entry.name_len].copy_from_slice(&entry.name[..entry.name_len]);
                // Fill rest with zeros
                for i in entry.name_len..MAX_FILENAME_LEN {
                    buf[count][i] = 0;
                }
                count += 1;
            }
        }
        count
    }

    pub fn get_file_metadata(&self, name: &str) -> Option<Metadata> {
        let entry = self.find_file_entry(name)?;
        if entry.inode_id >= MAX_INODES || !self.inodes[entry.inode_id].is_valid {
            return None;
        }
        Some(self.inodes[entry.inode_id].metadata)
    }

    pub fn find_inode_by_name(&self, name: &str) -> Option<usize> {
        self.find_file_entry(name).map(|e| e.inode_id)
    }

    pub fn delete_file(&mut self, name: &str) -> Result<(), &'static str> {
        let entry_idx = self.files.iter().position(|e| e.matches(name))
            .ok_or("File not found")?;

        let inode_id = self.files[entry_idx].inode_id;
        
        // Mark file entry as invalid
        self.files[entry_idx].is_valid = false;
        
        // Mark inode as invalid
        if inode_id < MAX_INODES {
            self.inodes[inode_id].is_valid = false;
        }

        Ok(())
    }
}
