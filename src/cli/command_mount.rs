extern crate clap;

use crate::mount::mount_repo;
use crate::repo::Repo;

/**
 * Snapshot mount
 */
pub fn execute(repo_path: &str, target_dir: &str) {
    let repo = Repo::new(repo_path);

    println!("Mounting repository {} in {}", &repo.path, &target_dir);
    mount_repo(&repo, target_dir);
}
