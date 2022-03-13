#![allow(unused)]
use std::{cell::RefCell, rc::Rc, rc::Weak};

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

#[cfg(test)]
mod tests {
    use super::{Gadget, Owner};
    use std::{cell::RefCell, rc::Rc};
    #[test]
    fn use_rc() {
        let gadget_owner: Rc<Owner> = Rc::new(Owner {
            name: "az".to_string(),
            gadgets: RefCell::new(vec![]),
        });

        let g1 = Rc::new(Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        });

        let g2 = Rc::new(Gadget {
            id: 2,
            owner: Rc::clone(&gadget_owner),
        });

        {
            let mut gadgets = gadget_owner.gadgets.borrow_mut();
            // downgrade to weak reference
            gadgets.push(Rc::downgrade(&g1));
            gadgets.push(Rc::downgrade(&g2));

            // `RefCell` dynamic borrow ends here
        }

        for gadget_weak in gadget_owner.gadgets.borrow().iter() {
            let gadget = gadget_weak.upgrade().unwrap();
            println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        }
    }

    // 验证局部可变性
    // 非mutable，却可以修改内部的refer_count
    #[test]
    fn rc_test() {
        use std::rc::Rc;
        let r1 = Rc::new(1);
        println!("Reference count {}", Rc::strong_count(&r1));
        let r2 = Rc::clone(&r1);
        println!("Reference count {}", Rc::strong_count(&r2));
    }

    #[test]
    fn cell_test() {
        use std::cell::Cell;
        let data: Cell<i32> = Cell::new(100);
        let p = &data;
        // use defer
        data.set(10);
        println!("{}", p.get());
        p.set(20);
        println!("{:?}", data);
    }

    #[test]
    fn ref_cell_test() {
        use std::cell::RefCell;
        let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
        let shared1 = &shared_vec;
        let shared2 = &shared1;

        shared1.borrow_mut().push(4);
        println!("{:?}", shared_vec.borrow());

        shared2.borrow_mut().push(5);
        println!("{:?}", shared_vec.borrow());
    }

    fn ref_cell_test_panik() {
        use std::cell::RefCell;
        let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 3, 4]);
        let shared1 = &shared_vec;
        let s2 = &shared1;
        // alias
        let p1 = shared1.borrow();
        let p2 = &p1[0];

        // borrow mutation, will panic
        s2.borrow_mut().push(4);
        println!("{}", p2);
    }
}
