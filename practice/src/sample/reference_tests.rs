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

    #[test]
    fn arc_test() {
        use std::sync::Arc;
        use std::thread;
        let numbers: Vec<_> = (0..100u32).collect();
        let shared_numbers_arc = Arc::new(numbers);

        for _ in 0..10 {
            let child_numbers = shared_numbers_arc.clone();
            thread::spawn(move || {
                let local_numbers = &child_numbers[..];
                println!("{:?}", local_numbers);
            });
        }
        println!("done");
    }

    #[test]
    fn mutex_test() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        const COUNT: u32 = 10000;

        let global = Arc::new(Mutex::new(0));
        let clone1 = global.clone();
        let thread1 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone1.lock().unwrap();
                *value += 1;
            }
        });

        let clone2 = global.clone();
        let thread2 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone2.lock().unwrap();
                *value -= 1;
            }
        });

        thread1.join().ok();
        thread2.join().ok();

        println!("final value: {:?}", global);
    }

    #[test]
    fn barrier_test() {
        use std::sync::{Arc, Barrier};
        use std::thread::{self, JoinHandle};

        let barrier = Arc::new(Barrier::new(10));
        let mut handlers: Vec<JoinHandle<()>> = vec![];

        for i in 0..10 {
            let c = barrier.clone();
            let t = thread::spawn(move || {
                println!("waiting {}", i);
                c.wait();
                println!("{} done", i);
            });
            handlers.push(t);
        }

        for h in handlers {
            h.join().ok();
        }
    }

    #[test]
    fn cond_var_test() {
        use std::sync::{Arc, Condvar, Mutex};
        use std::thread;
        use std::time::Duration;
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let p2 = pair.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let &(ref lock, ref cvar) = &*p2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            println!("child thread {}", *started);
        });

        let &(ref lock, ref cvar) = &*pair;
        let mut started = lock.lock().unwrap();

        println!("before wait {}", *started);

        while !*started {
            started = cvar.wait(started).unwrap();
        }

        println!("after wait {}", *started);
    }
}
