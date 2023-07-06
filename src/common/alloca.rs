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

#[test]
fn test_val() {
    use std::{collections::HashMap, mem::size_of_val};
    // 长度为0
    let c1 = || println!("hello world");
    // 长度为0
    let c2 = |i: i32| println!("hello {}", i);
    let name = String::from("ok");
    let name1 = name.clone();

    let mut table = HashMap::new();
    table.insert("hello", "world");

    // 捕获一个引用，长度为8
    let c3 = || println!("hello: {}", name);

    // 捕获移动的数据，name1的长度为24，table的长度为48，总共72
    let c4 = move || println!("hello: {}, {:?}", name1, table);

    let name2 = name.clone();

    // 捕获了 name2，closure 长度 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("not");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5)
    );
}
