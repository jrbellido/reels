use std::fs::create_dir_all;
use std::path::Path;

use crate::database::Database;
use crate::database::DatabaseSqlite;

pub struct Repo {
    pub path: String,
}

impl Repo {
    pub fn new(p: &str) -> Self {
        Self {
            path: String::from(p),
        }
    }

    pub fn init(&self) -> Result<(), &str> {
        if Path::new(&self.path).exists() {
            return Err("Directory exists");
        }
        create_dir_all(format!("{}/objects", self.path)).unwrap();
        Ok(())
    }

    pub fn snapshots(&self) {
        let index_file_path = format!("{}/index.db", self.path);
        let db = DatabaseSqlite::open(&index_file_path);
        let snapshots = db.find_snapshots();

        println!("Listing snapshots from {}:", self.path);
        for snapshot in snapshots {
            println!("[{}] created on {}", snapshot.id, snapshot.created_on);
        }
    }
}
