use crate::repo::Repo;

/**
 * Object list
 */
pub fn execute(repo_path: &str, object: &str) {
    match object {
        "snapshots" => {
            let repo = Repo::new(repo_path);
            repo.snapshots();
        }
        _ => {
            eprintln!("Not available object `{}`", object);
        }
    }
}
