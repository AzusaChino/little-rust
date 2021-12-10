#![allow(dead_code, unused)]

use std::process::{Command, Stdio};

const VERSION: &str = "1.0.0";

fn main() {
    println!("Welcome to little_rust, current version: {}", VERSION);
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}