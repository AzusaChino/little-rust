#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
struct A {
    a: u16,
    b: u8,
    c: bool,
}

#[test]
fn unsafe_convert() {
    let a = unsound_a();

    let some_a = Some(a);

    println!("a: {:#?}", a);
    println!("some_a: {:#?}", some_a);
}

fn unsound_a() -> A {
    #[derive(Debug, Copy, Clone)]
    struct B {
        a: u16,
        b: u8,
        c: u8,
    }
    let b = B { a: 1, b: 1, c: 1 };
    // the actual memory representation of bool is u8, but only for (0, 1)
    unsafe { *(&b as *const B as *const A) }
}

#[test]
fn double_free_possible() {
    let mut d = String::from("cccc");
    let d_len = d.len();
    {
        let mut c = String::with_capacity(d_len);
        unsafe {
            std::ptr::copy(&d, &mut c, 1);
        }
        println!("{:?}", c.as_ptr());
    }

    println!("{:?}", d.as_ptr());
    d.push_str("c");
    println!("{}", d);
}

#[test]
fn copy_on_heap() {
    use std::cell::RefCell;
    let a = Box::new(RefCell::new(1));
    let b = Box::new(RefCell::new(2));

    *b.borrow_mut() = *a.borrow();
    println!("b = {}", b.borrow());
}

mod other {
    struct A;

    // 没用，自己实现Copy和Clone无法改变编译器默认行为
    impl Clone for A {
        fn clone(&self) -> Self {
            println!("from Custom Copy: Clone");
            *self
        }
    }

    impl Copy for A {}

    #[test]
    fn custom_clone() {
        let a = A;
        let _ = a;
        // trigger customized clone behavior
        let _ = a.clone();
    }

    struct PrintDrop(&'static str);
    impl Drop for PrintDrop {
        fn drop(&mut self) {
            println!("Dropping {}", self.0)
        }
    }
    #[test]
    fn print_drop() {
        let _ = PrintDrop("x");
        let _ = PrintDrop("y");

        let _ = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
        let _ = (PrintDrop("x"), PrintDrop("y"), PrintDrop("z"));

        let z = PrintDrop("z");
        let x = PrintDrop("x");
        let y = PrintDrop("y");
        let _ = move || {
            drop(y);
            drop(z);
            drop(x);
        };
    }
}
