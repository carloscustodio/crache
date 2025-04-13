use crate::app::resp::{Resp, Value};
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration; // Import necessary types

pub struct Aof {
    file: Arc<RwLock<File>>,
}

impl Aof {
    pub fn new(file_path: &str) -> Self {
        // Open file with read+write permissions instead of just read
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .expect("Unable to open AOF file");

        let file = Arc::new(RwLock::new(file));

        // Spawn a thread that syncs the file to disk every 1 second.
        let file_clone = Arc::clone(&file);
        thread::spawn(move || loop {
            {
                let file_guard = file_clone.write().expect("Failed to acquire write lock");
                file_guard.sync_all().expect("Error syncing file");
            }
            thread::sleep(Duration::from_secs(1));
        });

        Aof { file }
    }

    // New method that reads and processes RESP values with a callback function
    pub fn read<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(Value),
    {
        let mut file_guard = self.file.write().expect("Failed to acquire write lock");

        // Seek to the beginning of the file
        file_guard.seek(SeekFrom::Start(0))?;

        // Create a copy of the file content in memory
        let mut buffer = Vec::new();
        file_guard.read_to_end(&mut buffer)?;

        // Create a cursor over the buffer
        let cursor = std::io::Cursor::new(buffer);

        // Create a Resp instance with the cursor
        let mut resp = Resp { reader: Ok(cursor) };

        loop {
            match resp.read() {
                Ok(value) => {
                    // Process the value with the callback
                    callback(value);
                }
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    // Reached end of file, break the loop
                    break;
                }
                Err(e) => {
                    // Return any other error
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    pub fn write(&self, data: &[u8]) -> Result<usize> {
        let mut file_guard = self.file.write().expect("Failed to acquire write lock");

        // Seek to end to append data
        file_guard.seek(SeekFrom::End(0))?;

        // Write the data and return bytes written
        file_guard.write(data)
    }

    pub fn sync(&self) -> Result<()> {
        let file_guard = self.file.write().expect("Failed to acquire write lock");
        file_guard.sync_all()
    }
}
