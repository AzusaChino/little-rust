//!
//! ```rs
//!     #[inline]
//!    fn clone(&self) -> Rc<T> {
//!       unsafe {
//!            self.inner().inc_strong();
//!            Self::from_inner(self.ptr)
//!        }
//!     }
//! ```

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;

    #[derive(Debug)]
    struct Node {
        id: usize,
        downstream: Option<Rc<Node>>,
    }

    impl Node {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                downstream: None,
            }
        }

        pub fn update_downstream(&mut self, downstream: Rc<Node>) {
            self.downstream = Some(downstream)
        }

        pub fn get_downstream(&self) -> Option<Rc<Node>> {
            self.downstream.as_ref().map(|v| v.clone())
        }
    }

    #[test]
    fn rc_test() {
        let a = Rc::new(1u64);
        let b = a.clone();
        let c = a.clone();

        let _ = b.is_power_of_two();
        let _ = c.leading_ones();
    }

    #[test]
    fn node_test() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);

        node3.update_downstream(Rc::new(node4));
        node1.update_downstream(Rc::new(node3));
        node2.update_downstream(node1.get_downstream().unwrap());
        println!("node1: {:?}, node2: {:?}", node1, node2.id);
    }

    #[test]
    fn refcell_test() {
        let data = RefCell::new(1);
        {
            // 获取 RefCell 内部数据的可变借用
            let mut v = data.borrow_mut();
            *v += 1;
        }
        println!("data: {:?}", data.borrow());
    }

    #[test]
    fn arc_test() {
        let s = Arc::new("abc".to_string());

        {
            let sc = s.clone();
            std::thread::spawn(move || {
                println!("{:?}", sc);
            });
        }
        println!("main {:?}", s);
    }
}

#[cfg(test)]
mod test2 {
    use std::cell::RefCell;
    use std::rc::Rc;
    #[derive(Debug)]
    struct Node {
        id: usize,
        // 使用 Rc<RefCell<T>> 让节点可以被修改
        downstream: Option<Rc<RefCell<Node>>>,
    }
    impl Node {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                downstream: None,
            }
        }
        pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
            self.downstream = Some(downstream);
        }
        pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
            self.downstream.as_ref().map(|v| v.clone())
        }
    }

    #[test]
    fn main() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);
        node3.update_downstream(Rc::new(RefCell::new(node4)));
        node1.update_downstream(Rc::new(RefCell::new(node3)));
        node2.update_downstream(node1.get_downstream().unwrap());
        println!("node1: {:?}, node2: {:?}", node1, node2);
        let node5 = Node::new(5);
        let node3 = node1.get_downstream().unwrap();
        // 获得可变引用，来修改 downstream
        node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));
        println!("node1: {:?}, node2: {:?}", node1.id, node2.id);
    }
}
