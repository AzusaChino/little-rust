#![allow(dead_code)]
use std::mem;

trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) {
        println!("duck");
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan")
    }
}

#[allow(unused)]
fn print_trait_object(ptr: &dyn Bird) {
    let (data, vtable): (usize, *const usize) = unsafe { mem::transmute(ptr) };
    println!("TraitObject [data:{}, vtable: {:p}]", data, vtable);

    unsafe {
        println!(
            "data in vtable [{}, {}, {}, {}]",
            *vtable,
            *vtable.offset(1),
            *vtable.offset(2),
            *vtable.offset(3)
        );
    }
}

#[test]
fn main() {
    let duck = Duck;
    let p_duck = &duck;
    // 胖指针
    let p_bird = p_duck as &dyn Bird;

    println!(
        "size of p_duck {}, size of p_bird {}",
        mem::size_of_val(p_duck),
        mem::size_of_val(p_bird),
    );

    let duck_fly: usize = Duck::fly as usize;
    let swan_fly: usize = Swan::fly as usize;

    println!("Duck::fly {}", duck_fly);
    println!("Swan::fly {}", swan_fly);

    print_trait_object(p_bird);
    let swan = Swan;
    print_trait_object(&swan as &dyn Bird);
}
