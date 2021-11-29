use std::mem::replace;

// Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.

// memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.
#[derive(Debug)]
struct Person {
    name: String,
    email: String,
}

fn main() {
    test_move();
    function_area();
}

fn test_move() {
    let p = Person {
        name: String::from("az"),
        email: String::from("aaa"),
    };

    // use move to steal p's name
    let _name = p.name;
    //   ------ value partially moved here
    println!("{}{}", _name, p.email);
    println!("{:?}", p);

    let __name = p.name.clone();
}

struct Buffer {
    buffer: String,
}

struct Render {
    current_buffer: Buffer,
    next_buffer: Buffer,
}

impl Render {
    fn update_buffer(&mut self, buf: String) {
        // self.current_buffer = self.next_buffer; // cannot move
        // self.next_buffer = Buffer { buffer: buf };
        // mem replace => std::swap
        self.current_buffer = replace(&mut self.next_buffer, Buffer { buffer: buf });
    }
}

fn function_area() {
    let r;
    {
        let x = 10;
        r = &x;
    }
    println!("{}", r)
    //   |
    // 52 |         r = &x;
    //    |             ^^ borrowed value does not live long enough
    // 53 |     }
    //    |     - `x` dropped here while still borrowed
    // 54 |     println!("{}", r)
    //    |                    - borrow later used here
}

fn test_int() {
    let x = 5;
    let y = x;
    // both valid
    println!("{}, {}", x, y)
    // integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
}

fn _main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn __mai() {
    let _s1 = gives_ownership();

    let s2 = String::from("hello");

    let _s3 = takes_and_give_back(s2);

    let (s4, l) = calculate_length(_s3);

    println!("{}, {}", s4, l)
}

fn gives_ownership() -> String {
    let some_string: String = String::from("abc");
    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}