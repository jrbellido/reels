extern crate clap;

use crate::profiler::Profiler;
use crate::repo::Repo;
use crate::snapshot::remove_snapshot;

/**
 * Snapshot deletion
 */
pub fn execute(repo_path: &str, snapshot_id: &str) {
    let p = Profiler::start();

    let repo = Repo::new(repo_path);

    println!("Removing snapshot...");
    remove_snapshot(repo, snapshot_id);
    println!("Snapshot {} was successfully deleted.", snapshot_id);
    p.stop_and_print();
}
