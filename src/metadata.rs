use std::fs::Metadata;
use std::time::UNIX_EPOCH;

use log::warn;

pub fn read_modified_date(metadata: &Metadata) -> Option<u64> {
    if let Ok(time) = metadata.modified() {
        Some(time.duration_since(UNIX_EPOCH).unwrap().as_secs())
    } else {
        warn!("WARN: Modification time not supported on this platform");
        None
    }
}

pub fn read_created_date(metadata: &Metadata) -> Option<u64> {
    if let Ok(time) = metadata.created() {
        Some(time.duration_since(UNIX_EPOCH).unwrap().as_secs())
    } else {
        warn!("WARN: Creation time not supported on this platform");
        None
    }
}
