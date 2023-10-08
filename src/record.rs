use std::path::Path;
use std::error::Error;

pub struct FileRecord {
    file_path: Path,
    file_hash: Vec<u8>
}

impl FileRecord {
    pub fn new(file_path: Path) -> Result<FileRecord, Box<dyn Error>> {
        let hash_string = sha256::try_digest(file_path)?;
        let hash_bytes = hash_string.as_slice().to_hex();

        Ok(hash_bytes)
    }
}
