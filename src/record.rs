use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    sync::Mutex,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRecord {
    path: &'static str,
    hash: Vec<u8>,
}

impl FileRecord {
    pub fn new(path: &'static str) -> Result<FileRecord, Box<dyn Error + '_>> {
        let hash_string = sha256::try_digest(Path::new(path))?;
        let hash = hash_string.as_bytes().to_vec();

        Ok(FileRecord { path, hash })
    }
}

pub struct FlatFileDB {
    file_path: &'static str,
    file_mutex: Mutex<File>,
    records: Vec<FileRecord>,
}

impl FlatFileDB {
    pub fn new(file_path: &'static str) -> Result<FlatFileDB, Box<dyn Error + '_>> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)?;

        Ok(FlatFileDB {
            file_path,
            file_mutex: Mutex::new(file),
            records: Vec::new(),
        })
    }

    pub fn close(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

    pub fn store(&mut self, record: FileRecord) -> Result<(), Box<dyn Error + '_>> {
        let record_encoded: Vec<u8> = bincode::serialize(&record)?;
        let mut file = self.file_mutex.lock()?;
        writeln!(*file, "{}", hex::encode(record_encoded))?;

        Ok(())
    }
}
