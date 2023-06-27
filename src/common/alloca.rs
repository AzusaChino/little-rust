//! 注意这里不能使用 println!() 。因为 stdout 会打印到一个由 Mutex 互斥锁保护的共享全
//! 局 buffer 中，这个过程中会涉及内存的分配，分配的内存又会触发 println!()，最终造成
//! 程序崩溃。而 eprintln! 直接打印到 stderr，不会 buffer。

use std::{
    alloc::{GlobalAlloc, System},
    fmt::Debug,
};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("ALLOC: {:p}, size {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        System.dealloc(ptr, layout);
        eprintln!("FREE: {:p}, size {}", ptr, layout.size());
    }
}

// 通知编译器，当前Allocator为全局Allocator
// #[global_allocator]
#[allow(dead_code)]
static GLOBAL: MyAllocator = MyAllocator;

struct Matrix {
    data: [u8; 505],
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

#[test]
fn main() {
    let data = Box::<Matrix>::default();
    // 输出中有一个 1024 大小的内存分配，是 println! 导致的
    println!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );
    drop(data);

    use std::io::Write;
    let mut buf: Vec<u8> = Vec::new();
    // dynamic dispatch
    let writer: &mut dyn Write = &mut buf;

    writer.write_all("hello world".as_bytes()).unwrap();
}
