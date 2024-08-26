#[cfg(test)]
mod tests {

    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    #[test]
    fn test() {
        println!("i32: {}", std::mem::size_of::<i32>());
        println!("rc<i32>: {}", std::mem::size_of::<Rc<i32>>());
        println!(
            "rc<RefCell<i32>>: {}",
            std::mem::size_of::<Rc<RefCell<i32>>>()
        );
        println!(
            "Option<rc<RefCell<i32>>>: {}",
            std::mem::size_of::<Option<Rc<RefCell<i32>>>>()
        );
    }

    #[derive(Debug, Default)]
    struct Structs {
        id: i32,
        age: Cell<i32>,
    }

    #[test]
    fn t() {
        let st = Structs::default();
        st.age.set(9);
        println!("{}, {:?}", st.id, st);
        let arr = [1, 2, 3, 4, 5];
        let addr = &arr;
        println!("addr {:p}", addr);
        raw_slice(addr);
    }

    fn raw_slice(arr: &[i32]) {
        unsafe {
            let (v1, v2): (usize, usize) = std::mem::transmute(arr);
            println!("v1: {:x}", v1);
            println!("v2: {:x}", v2);
        }
    }
}
