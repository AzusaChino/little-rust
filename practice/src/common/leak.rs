// Rust 内存泄漏：
// 1. 线程崩溃，析构函数无法调用
// 2. 使用引用计数时造成了循环引用
// 3. 调用Rust标准库中的forget函数主动泄露

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

    struct Node<T> {
        data: T,
        next: NodePtr<T>,
    }

    impl<T> Drop for Node<T> {
        fn drop(&mut self) {
            println!("node dropped!")
        }
    }

    #[test]
    fn main() {
        let a = Rc::new(RefCell::new(Node {
            data: 1,
            next: None,
        }));

        let b = Rc::new(RefCell::new(Node {
            data: 2,
            next: None,
        }));

        // 循环引用
        a.borrow_mut().next = Some(b.clone());
        b.borrow_mut().next = Some(a.clone());

        println!("{}", a.borrow().data);
    }
}
