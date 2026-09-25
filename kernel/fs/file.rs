#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileMode {
    Read,
    Write,
    ReadWrite,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileError {
    NotFound,
    PermissionDenied,
    InvalidOffset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileHandle {
    pub inode_id: u64,
    pub offset: u64,
    pub mode: FileMode,
}

impl FileHandle {
    pub const fn new(inode_id: u64, mode: FileMode) -> Self {
        Self {
            inode_id,
            offset: 0,
            mode,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, FileError> {
        let len = buffer.len().min(4096);
        self.offset = self.offset.saturating_add(len as u64);
        Ok(len)
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, FileError> {
        let len = buffer.len().min(4096);
        self.offset = self.offset.saturating_add(len as u64);
        Ok(len)
    }

    pub fn seek(&mut self, offset: u64) -> Result<u64, FileError> {
        if offset > 1 << 40 {
            return Err(FileError::InvalidOffset);
        }
        self.offset = offset;
        Ok(self.offset)
    }
}
