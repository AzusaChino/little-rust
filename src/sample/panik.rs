//! unwind 会一层层退出函数调用栈
//! abort 在发生panic时，会直接退出整个程序

#[test]
fn main() {
    std::panic::catch_unwind(|| {
        let x: Option<i32> = None;
        x.unwrap();
        println!("will not print out");
    })
    .ok();

    println!("back to main function");
}

#[test]
#[allow(unreachable_code)]
fn test() {
    let mut x: Vec<i32> = vec![1];
    let mut y: Vec<i32> = vec![2];

    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        x.push(10);
        panic!("user panic");
        y.push(100);
    }))
    .ok();

    println!("Observe corrupted data. {:?} {:?}", x, y);
}

#[test]
fn main_() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    const COUNT: u32 = 1000000;

    let global = Arc::new(Mutex::new(0));
    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            match clone1.lock() {
                Ok(mut value) => *value += 1,
                Err(e) => {
                    let mut val = e.into_inner();
                    *val += 1;
                }
            }
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone2.lock().unwrap();
            *value -= 1;
            if *value < 100000 {
                println!("make a panic");
                panic!("I mean it");
            }
        }
    });

    thread1.join().ok();
    thread2.join().ok();

    println!("final value : {:?}", global);
}
