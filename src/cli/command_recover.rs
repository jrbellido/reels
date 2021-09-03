extern crate clap;

use crate::profiler::Profiler;
use crate::recovery::recover_snapshot;
use crate::repo::Repo;

/**
 * Snapshot recovery
 */
pub fn execute(repo_path: &str, snapshot_id: &str, output_dir: &str) {
    let p = Profiler::start();
    let repo = Repo::new(repo_path);

    println!("Recovering from snapshot '{}'", &snapshot_id);
    match recover_snapshot(repo, snapshot_id, output_dir) {
        Ok(_) => println!("Recovery successfully completed."),
        Err(err) => eprintln!("Cannot recover from snapshot '{}': {}", &snapshot_id, err),
    };
    p.stop_and_print();
}
