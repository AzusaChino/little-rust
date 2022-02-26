#![allow(unused)]

use std::path::{Path, PathBuf};

pub trait Empty {}

#[derive(Clone)]
pub struct EmptyStruct;

impl Empty for EmptyStruct {}

fn simple_usage() {
    let mut path = PathBuf::with_capacity(10);
    let cap = path.capacity();

    path.push(r"C:\");

    assert_eq!(cap, path.capacity());

    assert_eq!(Path::new(r"C:\"), path.as_path());

    let mut path1 = PathBuf::from("/tmp");
    // with relative path, extend
    path1.push("file.bak");
    assert_eq!(path1, PathBuf::from("/tmp/file.bak"));

    let mut path2 = PathBuf::from("/tmp");
    // with absolute path, replace
    path2.push("/etc");
    assert_eq!(path2, PathBuf::from("/etc"));

    let mut path3 = PathBuf::from("/lib/x.rs");
    path3.pop();
    assert_eq!(path3, Path::new("/lib"));
    path3.pop();
    assert_eq!(path3, Path::new("/"));
}

pub fn create_es() -> Box<dyn Empty> {
    Box::new(EmptyStruct)
}

fn is_hello<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

fn test_is_hello() {
    let s = "hello".to_string();
    is_hello(s);
}
