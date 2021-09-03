use std::time::SystemTime;

use data_encoding::HEXLOWER;
use ring::digest::{self, SHA256};

pub fn from_timestamp() -> String {
    let timestamp: String = create_timestamp().to_string();
    let digest = digest::digest(&SHA256, &timestamp.as_bytes());
    String::from(HEXLOWER.encode(digest.as_ref()))
}

pub fn from_string(s: &String) -> String {
    let digest = digest::digest(&SHA256, &s.as_bytes());
    String::from(HEXLOWER.encode(digest.as_ref()))
}

pub fn from_bytes(b: &Vec<u8>) -> String {
    let digest = digest::digest(&SHA256, &b.to_vec());
    String::from(HEXLOWER.encode(digest.as_ref()))
}

pub fn create_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
