#[cfg(test)]
mod tests {

    use std::cell::{Cell, RefCell};

    #[test]
    fn test() {
        let data = Cell::new(100);
        let p = &data;

        data.set(1);
        println!("{:?}", p.get());

        p.set(2);
        println!("{:?}", data);
    }

    #[test]
    fn test_2() {
        let mut data = Cell::new(200);
        let p1 = data.get_mut();
        // let p2 = data.get_mut();
        // let p2 = p1.clone();

        *p1 = 3;
        println!("{:?}", data);

        // *p2 = 4;
        // println!("{:?}", data);
    }

    #[test]
    fn test_3() {
        let shared = RefCell::new(vec![1, 2, 3]);
        let s1 = &shared;
        let s2 = &s1;

        let p1 = s1.borrow();
        let p2 = &p1[0];
        s2.borrow_mut().push(4);
        println!("{:?}", p2);
    }
}
