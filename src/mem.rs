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
    }
}
