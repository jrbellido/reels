use std::fs;
use std::io::{Read, Result, Write};

use camino::Utf8Path;

use crate::chunk::{BLOCK_ID_LENGTH, PATH_LENGTH};
use crate::database::{Database, DatabaseSqlite};
use crate::path::path_from_hash;
use crate::reference::{Reference, ReferenceType};
use crate::repo::Repo;

pub fn recover_snapshot(repo: Repo, snapshot_id: &str, output_dir: &str) -> Result<()> {
    check_target_directory_does_not_exist(&output_dir)?;
    std::fs::create_dir_all(output_dir).unwrap();

    let index_file_path = format!("{}/index.db", repo.path);
    let db = DatabaseSqlite::open(&index_file_path);
    let snapshot = db.find_snapshot(snapshot_id);
    let snapshot_refs = db.find_references(&snapshot);

    for reference in snapshot_refs {
        recover_resource(&repo, &reference, &output_dir);
    }

    Ok(())
}

fn recover_resource(repo: &Repo, reference: &Reference, output_dir: &str) {
    let path = reference.path.replacen("/", "", 1).to_string();

    let joined = Utf8Path::new(output_dir)
        .join(Utf8Path::new(&path))
        .to_string();

    if reference.kind == ReferenceType::File {
        if reference.first_chunk_id.is_some() {
            copy_file_to_target(
                &repo,
                reference.first_chunk_id.as_ref().unwrap().to_string(),
                joined,
            );
        } else {
            std::fs::File::create(joined).unwrap();
        }
    } else {
        fs::create_dir_all(joined).unwrap_or_else(|err| {
            println!("{}", err);
        });
    }
}

fn check_target_directory_does_not_exist(output_dir: &str) -> Result<()> {
    if std::path::Path::new(output_dir).exists() {
        panic!("Target directory already exists.");
    }
    Ok(())
}

fn create_parent_dir(target_path: &str) {
    let path = std::path::Path::new(&target_path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
}

fn copy_file_to_target(repo: &Repo, chunk_id: String, target_path: String) {
    create_parent_dir(&target_path);

    let mut target_file = std::fs::File::create(target_path).unwrap();
    let mut source_path = path_from_hash(&chunk_id, PATH_LENGTH);
    let mut block_path = format!("{}/objects/{}/blob", repo.path, source_path);
    let mut source_file = std::fs::File::open(&block_path).unwrap();
    let mut next_register = vec![0; BLOCK_ID_LENGTH];
    let mut buffer = vec![0; 1024];

    'chunks: loop {
        // lets advance 64 bytes to read next chunk id
        source_file.read(&mut next_register).unwrap();

        // proceed to read blob in blocks and write them to target
        'blob: loop {
            let file_bytes = source_file.read(&mut buffer).unwrap();
            if file_bytes == 0 {
                break 'blob;
            }
            target_file.write(&buffer[0..file_bytes]).unwrap();
        }

        if is_zero_chunk(&next_register) {
            break 'chunks;
        } else {
            // prepare next block
            let next = &mut next_register;
            let next_str = std::str::from_utf8(next).unwrap();
            source_path = path_from_hash(next_str, PATH_LENGTH);
            block_path = format!("{}/objects/{}/blob", repo.path, source_path);
            source_file = std::fs::File::open(&block_path).unwrap();
        }
    }
}

fn is_zero_chunk(buffer: &Vec<u8>) -> bool {
    for f in buffer {
        if *f != 0 as u8 {
            return false;
        }
    }
    return true;
}
