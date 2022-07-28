#![allow(unused)]
use std::cmp::{Ordering, PartialOrd};

fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        b
    } else {
        a
    }
}

struct T {
    value: i32,
}

impl PartialOrd for T {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl PartialEq for T {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[test]
fn test() {
    let t1 = T { value: 1 };
    let t2 = T { value: 2 };
    let _m = max(t1, t2);
}
