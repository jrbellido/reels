use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

use crate::path::path_from_hash;
use crate::repo::Repo;

pub const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024;
pub const PATH_LENGTH: usize = 16;
pub const BLOCK_ID_LENGTH: usize = 64;
pub const EMPTY_BLOCK_ID: [u8; BLOCK_ID_LENGTH] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub struct Chunk {
    pub hash: String,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(data: Vec<u8>) -> Self {
        let hash_id = crate::hash::from_bytes(&data);
        Chunk {
            data,
            hash: hash_id,
        }
    }

    pub fn write_to_disk(&self, r: &Repo, next_id: Option<&String>) -> Result<i32> {
        let path = path_from_hash(&self.hash, PATH_LENGTH);
        let block_path = format!("{}/objects/{}", r.path, path);

        create_dir_all(&block_path)?;

        let block_file_path = format!("{}/blob", &block_path);

        // write only if blob does not exist
        if !Path::new(&block_file_path).is_file() {
            let mut file = File::create(&block_file_path).unwrap();

            // trace!("Writing chunk {}", &self.hash);

            match next_id {
                Some(id) => file.write_all(&id.as_bytes()),
                None => file.write_all(&EMPTY_BLOCK_ID),
            }
            .expect("Error writing chunk");

            file.write_all(&self.data)?;
            return Ok(self.data.len() as i32);
        }

        Ok(0)
    }
}

pub struct ChunkReader {
    source: File,
    block_size: usize,
}

impl ChunkReader {
    pub fn default(f: File) -> Self {
        Self::new(f, DEFAULT_CHUNK_SIZE)
    }

    pub fn new(f: File, block_size: usize) -> ChunkReader {
        ChunkReader {
            source: f,
            block_size,
        }
    }
}

impl Iterator for ChunkReader {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let mut reader = BufReader::with_capacity(self.block_size, &self.source);
        let buffer = reader.fill_buf().unwrap().to_vec();

        if buffer.len() > 0 {
            Some(Chunk::new(buffer.to_vec()))
        } else {
            None
        }
    }
}
