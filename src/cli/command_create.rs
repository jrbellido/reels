extern crate clap;

use tokio::runtime::Runtime;

use crate::profiler::Profiler;
use crate::repo::Repo;
use crate::snapshot::create_new_snapshot;

/**
 * Repository initialization
 */
pub fn execute(repo_path: &str, target_path: &str, root_path: Option<&str>) {
    let p = Profiler::start();
    let repo = Repo::new(repo_path);

    println!("Creating snapshot...");
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let snapshot = create_new_snapshot(repo, target_path, root_path).await;
        match snapshot {
            Ok(s) => println!("Created snapshot: {}", s.id),
            Err(e) => eprintln!("Error creating snapshot: {}", e),
        }
    });
    p.stop_and_print();
}
