extern crate humansize;

use humansize::{file_size_opts as options, FileSize};

pub struct Stats {
    bytes_written: i32,
    new_files: i32,
}

impl Stats {
    pub fn create() -> Self {
        Self {
            bytes_written: 0,
            new_files: 0,
        }
    }

    pub fn add_bytes_written(&mut self, byte_count: i32) {
        self.bytes_written += byte_count;
    }

    pub fn add_new_files(&mut self, file_count: i32) {
        self.new_files += file_count;
    }

    pub fn print(&self) {
        println!(
            "{} written, {} files created",
            self.bytes_written.file_size(options::CONVENTIONAL).unwrap(),
            self.new_files,
        );
    }
}
