#![allow(dead_code)]

use std::collections::HashSet;

fn main() {
    let mut s = String::from("foo");
    let t = String::from("bar");
    f(|| {
        s += &t;
        s
    });
}

fn f<F: FnOnce() -> String>(g: F) {
    println!("{}", g())
}

struct Closure<'a> {
    s: String,
    t: &'a String,
}

// impl<'a> FnOnce<()> for Closure<'a> {
//     type Output = String;

//     fn call_once(self, args: ()) -> Self::Output {
//         self.s + self.t + args()
//     }
// }

fn consume_with_relish<F>(func: F)
where
    F: FnOnce() -> String,
{
    println!("Consumed: {}", func());
}

struct SetVec {
    set: HashSet<u32>,
    vec: Vec<u32>,
}

impl SetVec {
    fn do_nothing(&mut self) {
        let mut x = 5;
        {
            // Unique immutable borrows in captures
            let mut square_x = || x *= x;
            square_x();
        }
        assert_eq!(x, 25)
    }

    fn tdb() {
        todo!("under construction")
    }

    fn populate(&mut self) {
        let vec = &mut self.vec;
        // capture reference
        self.set.iter().for_each(|&n| {
            vec.push(n);
        })
    }
}
