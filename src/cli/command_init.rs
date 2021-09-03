use crate::repo::Repo;

/**
 * Repository initialization
 */
pub fn execute(repo_path: &str) {
    let repo = Repo::new(repo_path);

    repo.init()
        .unwrap_or_else(|err| println!("Error: {:?}", err));
    println!("Repository initialized in \"{}\"", repo_path);
}
