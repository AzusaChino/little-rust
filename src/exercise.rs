use std::cell::Cell;
use std::path::PathBuf;
use std::process::{self};
use std::sync::{Arc, Mutex};

use crate::{Client, Connection};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const I_AM_DONE_REGEX: &str = r"(?m)^\s*///?\s*I\s+AM\s+NOT\s+DONE";
const CONTEXT: usize = 2;

// generate a temporary file name that is hopefully unique
#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    format!("./temp_{}_{}", process::id(), thread_id)
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Compile,
    Test,
    Clippy,
}

pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Debug)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub mode: Mode,
    pub hint: String,
}

fn test_client() -> Client {
    Client {
        conn: Arc::new(Mutex::new(Connection {})),
        txn: Cell::new(None),
    }
}

pub fn do_me_a_favor() {
    println!("do me a favor")
}