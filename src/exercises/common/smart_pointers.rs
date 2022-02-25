// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

use std::ops::Deref;

fn main() {
    // store an i32 value on the heap
    let b = Box::new(5);
    println!("{}", b)
}

// mod _a {
//     use self::List::{Cons, Nil};
//
//     enum List {
//         Cons(i32, List),
//         Nil,
//     }
//
//     fn l() {
//         // indefinite size
//         let _list = Cons(1, Cons(2, Cons(3, Nil)));
//     }
// }


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // deference
    assert_eq!(5, *y);

    let z = MyBox::new(3);
    assert_eq!(3, *z); // *(z.deref())

    let m = MyBox::new(String::from("rust"));
    println!("{}", &(*m)[..]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(x: String) -> Self {
        CustomSmartPointer { data: x }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping smp with data {}", self.data);
    }
}

// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!
fn test_drop() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    std::mem::drop(_c);
    println!("CustomSmartPointer dropped before the end of main.");
}

mod box_ {
    use self::List::{Cons, Nil};

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    fn t_b() {

        // stack size = i32 + box(usize)
        let __list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}


mod rc {
    use std::rc::Rc;

    use self::List::{Cons, Nil};

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    fn test_rc() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let _b = Cons(3, Rc::clone(&a));
        let _c = Cons(4, Rc::clone(&a));
    }

    fn __main__() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

mod ref_cell {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
        where
            T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::cell::RefCell;

        use super::*;

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![])
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                let mut one_borrow = self.sent_messages.borrow_mut();
                let mut two_borrow = self.sent_messages.borrow_mut();

                one_borrow.push(String::from(message));
                two_borrow.push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }
}

mod c {
    // 对外保持immutable, 内部可borrow_mut
    use std::cell::RefCell;
    // 引用计数
    use std::rc::Rc;

    use self::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    fn main() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

mod reference_cycle {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    use self::List::{Cons, Nil};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    fn main() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

    fn w_main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

mod sp {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    fn t_box() {
        // move value from stack to heap
        let val: u8 = 5;
        let _b: Box<u8> = Box::new(val);

        // move value from heap to stack by dereference
        let boxed: Box<u8> = Box::new(5);
        let _val: u8 = *boxed;
    }

    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    fn t_rc() {
        let gadget_owner: Rc<Owner> = Rc::new(
            Owner {
                name: "Gadget Man".to_string(),
                gadgets: RefCell::new(vec![]),
            }
        );

        let g1 = Rc::new(
            Gadget {
                id: 1,
                owner: Rc::clone(&gadget_owner),
            }
        );
        let g2 = Rc::new(
            Gadget {
                id: 2,
                owner: gadget_owner.clone(),
            }
        );

        {
            let mut gadgets = gadget_owner.gadgets.borrow_mut();
            gadgets.push(Rc::downgrade(&g1));
            gadgets.push(Rc::downgrade(&g2));
        }

        for gadget_weak in gadget_owner.gadgets.borrow().iter() {
            // `Weak` pointers can't guarantee the allocation still exists,
            // we need to call `upgrade`, which returns an `Option<Rc<Gadget>>`.
            let g = gadget_weak.upgrade().unwrap();
            println!("Gadget {} owned by {}", g.id, g.owner.name);
        }
    }
}