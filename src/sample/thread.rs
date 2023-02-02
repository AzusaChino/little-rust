#[cfg(test)]
mod test {

    use std::cell::RefCell;
    use std::thread;

    // 局部变量测试
    #[test]
    fn main() {
        // 使用thread_local!声明的变量，使用时需要with() 方法加闭包
        thread_local! {
            static FOO: RefCell<u32> = RefCell::new(1);
        }

        FOO.with(|f| {
            println!("main thread value1: {:?}", *f.borrow());
            *f.borrow_mut() = 2;
            println!("main thread value2: {:?}", *f.borrow());
        });

        let t = thread::spawn(move || {
            FOO.with(|f| {
                println!("child thread value1: {:?}", *f.borrow());
                *f.borrow_mut() = 3;
                println!("child thread value2: {:?}", *f.borrow());
            });
        });

        t.join().ok();

        FOO.with(|f| {
            println!("main thread value3: {:?}", *f.borrow());
        });
    }
}
