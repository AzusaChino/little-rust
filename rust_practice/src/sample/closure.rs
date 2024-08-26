#![allow(unused)]

fn sample() {
    let add = |a: i32, b: i32| -> i32 { a + b };
    let x = add(1, 2);
    println!("result is {}", x);
}

fn wrong_sample() {
    let x = 1_i32;
    fn inner_add() -> i32 {
        // can't capture dynamic environment in a fn item
        // x+1 // won't compile
        1
    }
    let x2 = inner_add();
    println!("result is {}", x2);
}

fn another_sample() {
    let x = 1_i32;
    // closure will capture outside variable
    let inner_add = || x + 1;
    let x2 = inner_add();
    println!("result is {}", x2);
}

// 不需要捕获环境变量的场景，普通函数fn和closure可以互换
fn third_sample() {
    let option = Some(2);
    let new: Option<i32> = option.map(multiple2);
    println!("{:?}", new);

    fn multiple2(val: i32) -> i32 {
        val * 2
    }
}

// 1. 如果一个外部变量在闭包中，只通过借用指针&使用，那么这个变量就可以通过引用&的方式捕获
// 2. 如果一个外部变量在闭包中，通过&mut指针使用过，那么这个变量需要使用&mut方式捕获
// 3. 如果一个外部变量在闭包中，通过所有权转移的方式使用过，那么这个变量需要使用 `by value` self 的方式捕获
struct Closure {
    inner: i32,
}

impl Closure {
    fn call(&self, a: i32) -> i32 {
        self.inner + a
    }
}

fn closure_imm() {
    let x = 1_i32;
    let add_x = Closure { inner: x };
    let result = add_x.call(5);
    println!("result is {}", result);
}

struct T(i32);

fn by_value(_: T) {}
fn by_mut(_: &mut T) {}
fn by_ref(_: &T) {}

fn use_t() {
    let x: T = T(1);
    let y: T = T(2);
    let mut z: T = T(3);

    let closure = || {
        by_value(x);
        by_ref(&y);
        by_mut(&mut z);
    };

    closure();
}

fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    // error: closure may outlive the current function `x`
    // Box::new(|y| x + y)

    Box::new(move |y| x + y)
}
