use std::convert::Into;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use log::{debug, trace};

use crate::errors::*;

/// Naively replaces any path separator with a forward-slash "/"
pub fn normalize_path(path: &str) -> String {
    use std::path::is_separator;
    path.chars()
        .map(|ch| if is_separator(ch) { '/' } else { ch })
        .collect::<String>()
}

/// Write the given data to a file, creating it first if necessary
pub fn write_file<P: AsRef<Path>>(build_dir: &Path, filename: P, content: &[u8]) -> Result<()> {
    let path = build_dir.join(filename);
    create_file(&path)?.write_all(content).map_err(Into::into)
}

pub fn create_file(path: &Path) -> Result<File> {
    debug!("Creating {}", path.display());

    if let Some(p) = path.parent() {
        trace!("Parent directory is: {:?}", p);

        fs::create_dir_all(p)?;
    }
    File::create(path).map_err(Into::into)
}

/// Takes a path and returns a path containing just enough `../` to point to
/// the root of the given path.
///
/// This is mostly interesting for a relative path to point back to the
/// directory from where the path starts.
///
/// ```rust
/// # use std::path::Path;
/// # use mdbook::utils::fs::path_to_root;
/// let path = Path::new("some/relative/path");
/// assert_eq!(path_to_root(path), "../../");
/// ```
pub fn path_to_root<P: Into<PathBuf>>(path: P) -> String {
    path
        .into()
        .parent()
        .expect("")
        .components()
        .fold(String::new(), |mut s, c| {
            match c {
                Component::Normal(_) => s.push_str("../"),
                _ => {
                    debug!("Other path component... {:?}", c);
                }
            }
            s
        })
}

pub fn remove_dir_content(dir: &Path) -> Result<()> {
    for item in fs::read_dir(dir)? {
        if let Ok(item) = item {
            let item = item.path();
            if item.is_dir() {
                fs::remove_dir_all(item)?;
            } else {
                fs::remove_file(item);
            }
        }
    }
    Ok(())
}


pub fn get_404_output_file(intput_404: &Option<String>) -> String {
    intput_404
        .as_ref()
        .unwrap_or(&"404.md".to_string())
        .replace(".md", ".html")
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Result, path::Path};
    use std::path::PathBuf;

    use crate::utils::fs::{create_file, path_to_root, write_file};

    use super::normalize_path;

    #[cfg(target_os = "windows")]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
        std::os::windows::fs::symlink_file(src, dst)
    }

    #[cfg(not(target_os = "windows"))]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
        std::os::unix::fs::symlink(src, dst)
    }

    #[test]
    fn test_normalize_path() {
        let path = std::env::current_dir().unwrap();
        println!("{}", normalize_path(path.to_str().unwrap()));
    }

    #[test]
    fn test_create_file() {
        let build_dir = std::env::current_dir().unwrap();
        let filename = PathBuf::from("abc.txt");
        let content = "haha".as_bytes();
        write_file(build_dir.as_path(), filename, content);
    }

    #[test]
    fn test_path_to_root() {
        let p = std::env::current_dir().unwrap();
        println!("{}", path_to_root(p));
    }
}
