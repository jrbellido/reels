use camino::Utf8Path;

use metadata::{read_created_date, read_modified_date};

use crate::hash::from_string;
use crate::metadata;
use crate::path::normalize_path;

#[derive(Debug, PartialEq)]
pub enum ReferenceType {
    File,
    Directory,
}

#[derive(Debug)]
pub struct Reference {
    pub id: String,
    pub kind: ReferenceType,
    pub path: String,
    pub original_path: String,
    pub first_chunk_id: Option<String>,
}

impl Reference {
    pub fn new(path: &Utf8Path, chroot: &str) -> Self {
        let metadata = path.metadata().unwrap();

        let normalized_target_path = normalize_path(path, chroot).unwrap();

        let reference_key = format!(
            "{}|{}|{}|{}",
            &normalized_target_path,
            metadata.is_file(),
            read_created_date(&metadata).unwrap(),
            read_modified_date(&metadata).unwrap()
        );
        let hash_id = from_string(&reference_key);

        let kind = match metadata.is_file() {
            true => ReferenceType::File,
            false => ReferenceType::Directory,
        };

        Self {
            id: hash_id,
            kind,
            path: normalized_target_path,
            original_path: normalize_path(path, "").unwrap(),
            first_chunk_id: None,
        }
    }

    pub fn from(
        id: String,
        kind: ReferenceType,
        path: String,
        original_path: String,
        first_chunk_id: Option<String>,
    ) -> Self {
        Self {
            id,
            kind,
            path,
            original_path,
            first_chunk_id,
        }
    }
}
