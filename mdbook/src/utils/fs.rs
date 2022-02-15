use std::convert::Into;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use crate::errors::*;

/// Naively replaces any path separator with a forward-slash "/"
pub fn normalize_path(path: &str) -> String {
    use std::path::is_separator;
    path.chars()
        .map(|ch| if is_separator(ch) { '/' } else { ch })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Result, path::Path};

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
}
