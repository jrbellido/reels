use std::fs::File;
use std::io::Result;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use chrono::Utc;
use futures::stream::{self, StreamExt};
use tokio::sync::Mutex;

use crate::chunk::{Chunk, ChunkReader};
use crate::database::{Database, DatabaseSqlite};
use crate::hash::from_timestamp;
use crate::reference::{Reference, ReferenceType};
use crate::repo::Repo;
use crate::stats::Stats;

pub struct SnapshotRequest {
    pub snapshot_id: String,
    pub target_path: String,
    pub root_path: String,
}

impl SnapshotRequest {
    pub fn from(snapshot_id: &str, target: &str, root: Option<&str>) -> Self {
        let root_path = { root.as_ref().map_or("", |x| x) }.to_string();
        Self {
            snapshot_id: snapshot_id.to_string(),
            target_path: target.to_string(),
            root_path,
        }
    }
}

pub struct Snapshot {
    pub id: String,
    pub created_on: String,
}

impl Snapshot {
    pub fn from_database(id: String, created_on: String) -> Self {
        Self { id, created_on }
    }
}

pub async fn create_new_snapshot(repo: Repo, target: &str, root: Option<&str>) -> Result<Snapshot> {
    let snapshot_id = from_timestamp();
    let created_on = Utc::now().format("%Y-%m-%d %H:%M:%S");

    let snapshot_req = Arc::new(SnapshotRequest::from(snapshot_id.as_str(), target, root));
    let wd = walkdir::WalkDir::new(&target).into_iter();

    let index_file_path = format!("{}/index.db", repo.path);
    let async_db = Arc::new(Mutex::new(DatabaseSqlite::open(&index_file_path)));
    let async_stats = Arc::new(Mutex::new(Stats::create()));

    // Begin transaction
    {
        let db_state = Arc::clone(&async_db);
        let db = db_state.lock().await;
        db.start_transaction();
    }

    let async_repo = Arc::new(repo);

    let mut work = stream::iter(wd)
        .map(|v| {
            let db_state = Arc::clone(&async_db);
            let repo_state = Arc::clone(&async_repo);
            let stats_state = Arc::clone(&async_stats);
            let snapshot_state = Arc::clone(&snapshot_req);
            tokio::spawn(async move {
                match v {
                    Ok(e) => {
                        let entry_path_string = e.path().to_str().unwrap().to_string();
                        let entry_path = Utf8Path::new(&entry_path_string);
                        let reference = Reference::new(entry_path, &snapshot_state.root_path);

                        let ref_exists = {
                            let db = db_state.lock().await;
                            db.reference_exists(&reference.id)
                        };
                        let mut stats = { stats_state.lock().await };

                        if !ref_exists {
                            let first_chunk = write_reference_to_disk(
                                &repo_state,
                                &reference,
                                &entry_path.to_path_buf(),
                                &mut stats,
                            );
                            {
                                let db = db_state.lock().await;
                                let is_file = match reference.kind {
                                    ReferenceType::File => {
                                        stats.add_new_files(1);
                                        1 as i64
                                    }
                                    ReferenceType::Directory => 0 as i64,
                                };
                                db.store_head(&reference, is_file, first_chunk);
                            }
                            println!("Added '{}'", &reference.path);
                        }
                        {
                            let db = db_state.lock().await;
                            db.store_reference(&reference, &snapshot_state.snapshot_id);
                        }
                    }
                    Err(_) => {}
                }
            })
        })
        .buffer_unordered(4);
    while let Some(_res) = work.next().await {}

    let snapshot = Snapshot::from_database(snapshot_id.to_string(), created_on.to_string());

    // End transaction
    {
        let db = Arc::clone(&async_db);
        let db_lock = db.lock().await;
        db_lock.store_snapshot(&snapshot);
        db_lock.end_transaction();
    }

    // Print stats
    {
        let stats_lock = async_stats.lock().await;
        stats_lock.print();
    }

    Ok(snapshot)
}

fn write_reference_to_disk(
    repo: &Repo,
    reference: &Reference,
    path: &Utf8PathBuf,
    stats: &mut Stats,
) -> Option<String> {
    match reference.kind {
        ReferenceType::File => Some(write_file_in_chunks(&repo, path, stats)),
        ReferenceType::Directory => None,
    }
}

fn write_file_in_chunks(repo: &Repo, path: &Utf8Path, stats: &mut Stats) -> String {
    let f = File::open(&path).unwrap();

    let mut reader = ChunkReader::default(f);

    let mut prev_chunk: Chunk;
    let mut first_chunk_id = "".to_string();

    if let Some(first_chunk) = reader.next() {
        first_chunk_id = first_chunk.hash.to_string();
        prev_chunk = first_chunk;
        while let Some(chunk) = reader.next() {
            let written_bytes = prev_chunk.write_to_disk(&repo, Some(&chunk.hash)).unwrap();
            stats.add_bytes_written(written_bytes);
            prev_chunk = chunk;
        }
        let written_bytes = prev_chunk.write_to_disk(&repo, None).unwrap();
        stats.add_bytes_written(written_bytes);
    }

    return first_chunk_id;
}

pub fn remove_snapshot(repo: Repo, snapshot_id: &str) {
    let index_file_path = format!("{}/index.db", repo.path);
    let db = DatabaseSqlite::open(&index_file_path);

    let snapshot = db.find_snapshot(snapshot_id);
    db.start_transaction();
    db.remove_snapshot(&snapshot);
    db.end_transaction();
}
