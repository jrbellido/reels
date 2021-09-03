use camino::Utf8Path;

pub fn path_from_hash(s: &str, length: usize) -> String {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(length)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}

/*
* Returns the absolute (canonical) path to the given relative path. It also removes
* the root from path if passed.
*
* Examples:
*      path = /Volumes/DATA/photos        chroot = /Volumes/DATA    result = /photos
*      path = ../Workspace                chroot = /Users/jr        result = /Workspace
*      path = ../Workspace                chroot =                  result = /Users/jr/Workspace
*/
pub fn normalize_path(path: &Utf8Path, chroot: &str) -> Result<String, ()> {
    let path_str = path
        .canonicalize()
        .unwrap()
        .as_path()
        .to_str()
        .unwrap()
        .to_string();

    if chroot.len() > 0 {
        let chroot_str = Utf8Path::new(&chroot)
            .canonicalize()
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_string();
        return Ok(path_str.replacen(&chroot_str, "", 1));
    } else {
        return Ok(path_str);
    }
}
