use std::sync;

pub fn echo(s: &str) {
    println!("{}", s)
}

pub struct Ctrl {
    lock: sync::Mutex<u64>,
}

impl Ctrl {
    pub fn run<F: Fn()>(self, f: F) {
        f();
    }
}

pub fn run<F: Fn()>(f: F) {
    f()
}