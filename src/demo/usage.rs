use super::Person;
use super::Sex;

fn use_super() {
    let _p = Person {
        name: "".to_string(),
        age: 0,
        sex: Sex::MALE,
        email: "".to_string(),
    };
}

mod st {
    trait Printer {
        fn print(&self);
    }

    struct Console;

    impl Printer for Console {
        fn print(&self) {
            println!("I am a console")
        }
    }

    fn call_printer(p: &'static dyn Printer) {
        p.print()
    }

    fn lifecycle<'a>(a: &'a str, b: &'a str) {
        println!("{}{}", a, b)
    }
}

mod cl {
    use std::fmt::Display;
    use std::thread;

    fn a() {
        let _print = |x: Box<dyn Display>| {
            println!("{}", x);
        };

        let _print_move = move |x: String| {
            println!("{}", x);
        };

        let s = String::from("oops");
        thread::spawn(move || {
            println!("{}", s);
        });

        // unknown memory size
        // let _pp = |f: dyn Fn()| {
        //     f();
        // };
    }

    fn do_twice<F>(mut func: F)
        // prefered Fn(usize) -> (usize)
        where F: FnMut() -> () {
        func();
    }
}
