#[cfg(test)]
mod tests {
    extern crate lazy_static;
    extern crate regex;

    // created only the first time it is used
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{collections::HashMap, fmt::Display, ops::MulAssign, sync::RwLock};

    // global static immutable
    lazy_static! {
        static ref CURRENCIES: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert("EUR", "Euro");
            m.insert("USD", "U.S. Dollar");
            m.insert("CNY", "RMB");
            m
        };
    }

    // global static mutable
    lazy_static! {
        static ref CLIENTS: RwLock<Vec<String>> = RwLock::new(Vec::new());
    }

    // local static
    fn extract_day(date: &str) -> Option<&str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(\d{2}).(\d{2}).(\d{4})").expect("fail to create regex");
        }

        RE.captures(date)
            .and_then(|cap| cap.get(1).map(|day| day.as_str()))
    }

    #[test]
    fn lazy() {
        let usd = CURRENCIES.get("USD");
        if let Some(usd) = usd {
            println!("{}", usd);
        }

        // mutate the global
        CLIENTS
            .write()
            .expect("fail to unlock clients for writing")
            .push("192.168.0.1".to_owned());

        let clients = CLIENTS.read().expect("fail to get read lock");
        println!("{}", clients.get(0).expect("fail to get first"));

        let date = "12.01.2018";
        if let Some(day) = extract_day(date) {
            println!("{}", day);
        }
    }

    extern crate bitflags;

    bitflags::bitflags! {
        struct Spices: u32 {
            const SALT = 0b0000_0001;
            const PEPPER = 0b0000_0010;
            const CHILI = 0b0000_0100;
            const SAFFRON = 0b0000_1000;
            const ALL = Self::SALT.bits
            |Self::PEPPER.bits
            |Self::CHILI.bits
            |Self::SAFFRON.bits;
        }
    }

    impl Spices {
        pub fn clear(&mut self) -> &mut Self {
            self.bits = 0;
            self
        }
    }

    #[test]
    fn bit_fields() {
        let classic = Spices::SALT | Spices::PEPPER;
        let spicy = Spices::PEPPER | Spices::CHILI;

        println!("classic {:?}", classic);
        // 00000011
        println!("classic {:08b}", classic.bits());

        println!("spicy {:?}", spicy);
        // 00000110
        println!("spicy {:08b}", spicy.bits());

        let mut custom = classic | spicy;
        custom.insert(Spices::SAFFRON);
        println!("{:?}", custom);

        custom.toggle(Spices::CHILI);
        println!("{:?}", custom);

        custom.remove(Spices::SALT);
        println!("{:?}", custom);

        custom.clear();
    }

    #[derive(Debug)]
    struct DoubleVec<T>(Vec<T>);

    // allowing conversion from a Vec<T>
    impl<T> From<Vec<T>> for DoubleVec<T>
    where
        T: MulAssign<i32>,
    {
        fn from(mut value: Vec<T>) -> Self {
            for elm in &mut value {
                *elm *= 2;
            }
            DoubleVec(value)
        }
    }

    // allowing conversion from a slice<'a, T>
    impl<'a, T> From<&'a [T]> for DoubleVec<T>
    where
        T: MulAssign<i32> + Clone,
    {
        fn from(value: &[T]) -> Self {
            value.to_vec().into()
        }
    }

    // allowing conversion from a DoubleVec to a vec
    impl<T> AsRef<Vec<T>> for DoubleVec<T> {
        fn as_ref(&self) -> &Vec<T> {
            &self.0
        }
    }

    #[test]
    fn cov() {
        let _ = "hello world".to_string();
        let _: String = "hello world".into();
        let _: String = String::from("hello world");

        let vec = vec![1, 2, 3];
        let _ = DoubleVec::from(vec.clone());
        let _: DoubleVec<i32> = vec.clone().into();
        let dv: DoubleVec<i32> = vec[..].into();
        print_elm(dv.as_ref());
    }

    fn print_elm<T>(slc: &[T])
    where
        T: Display,
    {
        for el in slc {
            print!("{} ", el);
        }
        println!();
    }

    #[derive(Debug)]
    struct Node<T>
    where
        T: std::fmt::Display,
    {
        pub(crate) data: T,
        child_nodes: Option<(BoxedNode<T>, BoxedNode<T>)>,
    }

    type BoxedNode<T> = Box<Node<T>>;

    impl<T> Node<T>
    where
        T: std::fmt::Display,
    {
        fn new(data: T) -> Self {
            Self {
                data,
                child_nodes: None,
            }
        }

        fn is_leaf(&self) -> bool {
            self.child_nodes.is_none()
        }

        fn add_child_nodes(&mut self, a: Node<T>, b: Node<T>) {
            assert!(
                self.is_leaf(),
                "tried to add child_nodes to a node that is not a leaf"
            );
            self.child_nodes = Some((Box::new(a), Box::new(b)));
        }

        fn print(&self) {
            println!("{}", self.data);
        }
    }

    trait Animal: std::fmt::Debug {
        fn sound(&self) -> &'static str;
    }

    #[derive(Debug, Default)]
    struct Dog;
    impl Animal for Dog {
        fn sound(&self) -> &'static str {
            "Woof!"
        }
    }

    #[derive(Debug, Default)]
    struct Cat;
    impl Animal for Cat {
        fn sound(&self) -> &'static str {
            "Meow!"
        }
    }
    #[test]
    fn boks() {
        let mut root = Node::new(12);
        root.add_child_nodes(Node::new(3), Node::new(4));
        root.child_nodes
            .as_mut()
            .expect("none")
            .0
            .add_child_nodes(Node::new(0), Node::new(1903));

        root.print();

        println!("{:?}", root);

        let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
        zoo.push(Box::new(Dog::default()));
        zoo.push(Box::new(Cat::default()));
        println!("{:?}", zoo);
    }
}
