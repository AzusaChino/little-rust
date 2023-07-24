#[cfg(test)]
mod tests {
    extern crate regex;
    use once_cell::sync::Lazy;
    use std::{collections::HashMap, sync::Mutex};
    use std::{fmt::Display, ops::MulAssign};

    extern crate bitflags;

    static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert(13, "Spica".to_owned());
        m.insert(74, "Hoyten".to_owned());
        Mutex::new(m)
    });

    bitflags::bitflags! {
        struct Spices: u32 {
            const NONE = 0b0000_0000;
            const SALT = 0b0000_0001;
            const PEPPER = 0b0000_0010;
            const CHILI = 0b0000_0100;
            const SAFFRON = 0b0000_1000;
            const ALL = Self::SALT.bits()
            |Self::PEPPER.bits()
            |Self::CHILI.bits()
            |Self::SAFFRON.bits();
        }
    }

    impl std::fmt::Debug for Spices {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Spices").field(&self.0).finish()
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

        GLOBAL_DATA.is_poisoned();
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

        // convert to vec!
        let zoo: Vec<Box<dyn Animal>> = vec![Box::<Dog>::default(), Box::<Cat>::default()];
        println!("{:?}", zoo);
    }
}
