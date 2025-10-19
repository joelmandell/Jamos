// PostgreSQL-inspired metadata-rich filesystem
// Each file/directory has rich metadata similar to PostgreSQL's system catalogs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
}

#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    pub size: usize,
    pub file_type: FileType,
    pub permissions: u16,
    pub created_at: u64,    // Timestamp (simple counter for now)
    pub modified_at: u64,   // Timestamp
    pub owner_id: u16,
    pub group_id: u16,
}

impl Metadata {
    pub const fn new(file_type: FileType) -> Self {
        Metadata {
            size: 0,
            file_type,
            permissions: 0o644,
            created_at: 0,
            modified_at: 0,
            owner_id: 0,
            group_id: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Inode {
    pub id: usize,
    pub metadata: Metadata,
    pub data_offset: usize,  // Offset in data storage
    pub parent_id: usize,    // Parent directory inode
    pub is_valid: bool,
}

impl Inode {
    pub const fn empty() -> Self {
        Inode {
            id: 0,
            metadata: Metadata::new(FileType::Regular),
            data_offset: 0,
            parent_id: 0,
            is_valid: false,
        }
    }

    pub fn new(id: usize, file_type: FileType, parent_id: usize) -> Self {
        Inode {
            id,
            metadata: Metadata::new(file_type),
            data_offset: 0,
            parent_id,
            is_valid: true,
        }
    }
}
