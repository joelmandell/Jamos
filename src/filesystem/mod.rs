pub mod vfs;
pub mod metadata;

pub use vfs::{VirtualFileSystem, FileHandle};
pub use metadata::{Inode, FileType, Metadata};
