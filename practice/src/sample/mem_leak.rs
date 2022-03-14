#![allow(unused)]

//! 1. 使用引用计数指针
//! 2. 存在内部可变性
//! 3. 指针所指向的内容本身不是 'static 的

// 验证Rust内存泄漏的过程

#[cfg(test)]
mod first_version {
    struct Node {
        next: Option<Box<Node>>,
    }

    #[test]
    fn test() {
        let mut node1 = Box::new(Node { next: None });
        let mut node2 = Box::new(Node { next: None });
        node1.next = Some(node2);
        // assign to part of moved value node2 ↓
        // node2.next = Some(node1);
    }

    #[test]
    fn test2() {
        let mut node1 = Box::new(Node { next: None });
        let mut node2 = Box::new(Node { next: None });
        node1.next = Some(node2);
        match node1.next {
            // use of partially moved value `node1` ↓
            // Some(mut n) => n.next = Some(node1),
            Some(mut n) => {}
            None => {}
        }
    }
}

#[cfg(test)]
mod second_version {
    use std::rc::Rc;

    struct Node {
        next: Option<Rc<Node>>,
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("drop");
        }
    }

    #[test]
    fn test() {
        // 栈上分配
        let mut node1 = Node { next: None };
        let mut node2 = Node { next: None };
        let mut node3 = Node { next: None };
        node1.next = Some(Rc::new(node2));
        // assign of moved value
        // node2.next = Some(Rc::new(node3));
        // node3.next = Some(Rc::new(node1));
    }

    #[test]
    fn test2() {
        // 堆上分配
        let mut node1 = Rc::new(Node { next: None });
        let mut node2 = Rc::new(Node { next: None });
        let mut node3 = Rc::new(Node { next: None });
        // cannot assign to data in an `Rc` trait `DerefMut` is required
        // node1.next = Some(node2);
    }
}

#[cfg(test)]
mod third_version {
    use std::{cell::RefCell, rc::Rc};

    struct Node {
        next: Option<Rc<RefCell<Node>>>,
    }
    impl Node {
        fn new() -> Self {
            Self { next: None }
        }
    }
    impl Drop for Node {
        fn drop(&mut self) {
            println!("drop");
        }
    }

    #[test]
    fn test() {
        alloc();
        println!("program finished");
    }

    // 循环引用, 且编译器无错误
    fn alloc() {
        let mut node1 = Rc::new(RefCell::new(Node::new()));
        let mut node2 = Rc::new(RefCell::new(Node::new()));
        let mut node3 = Rc::new(RefCell::new(Node::new()));
        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node1.clone());
    }
}
