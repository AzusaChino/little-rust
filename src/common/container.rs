#[cfg(test)]
mod tests {
    struct Integer(u32);
    type Int = i32;

    #[test]
    fn main_struct() {
        let int = Integer(10);
        assert_eq!(int.0, 10);
        let int: Int = 10;
        assert_eq!(int, 10);
    }

    struct Empty;

    #[test]
    fn main_empty() {
        // 在 Release 编译模式下，单元结构体实例会被优化为同一个对象。
        let x = Empty;
        println!("{:p}", &x);

        let y = x;
        println!("{:p}", &y);

        let z = Empty;
        println!("{:p}", &z);
        assert_eq!((..), std::ops::RangeFull);
    }

    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    #[test]
    fn main_color() {
        println!("roses are ${:06x}", Color::Red as i32);
        println!("leaves are ${:06x}", Color::Green as i32);
        println!("violets are ${:06x}", Color::Blue as i32);
    }

    use std::collections::{BTreeMap, BinaryHeap, HashMap};

    #[test]
    fn main_bh() {
        let mut heap = BinaryHeap::new();
        assert_eq!(heap.peek(), None);

        let arr = [1, 22, 39, 46, 54, 61, 27, 8, 823];
        for &i in arr.iter() {
            heap.push(i);
        }
        assert_eq!(heap.peek(), Some(&823));
        println!("{:?}", heap);
    }

    #[test]
    fn create_boxt() {
        let mut v1 = vec![1, 2, 3, 4];
        v1.push(5);
        println!("cap should be 8: {}", v1.capacity());

        // 从 Vec<T> 转换成 Box<[T]>，此时会丢弃多余的 capacity
        let b1 = v1.into_boxed_slice();
        let mut b2 = b1.clone();

        let v2 = b1.into_vec();
        println!("cap should be exactly 5: {}", v2.capacity());

        // Box<T> 可以更改内部数据，无法push
        b2[0] = 2;
        println!("b2: {:?}", b2);

        // 注意 Box<[T]> 和 Box<[T; n]> 并不相同
        let b3 = Box::new([2, 2, 3, 4, 5]);
        println!("b3: {:?}", b3);
        // b2 和 b3 相等，但 b3.deref() 和 v2 无法比较
        assert!(b2 == b3);
        // assert!(b3.deref() == v2);
    }

    // IteratorExt 「继承」Iterator，这样可以使用 Iterator 的全部功能
    pub trait IteratorExt: Iterator {
        fn window_count(self, count: u32) -> WindowCount<Self>
        where
            Self: Sized,
        {
            WindowCount { iter: self, count }
        }
    }

    // 这句很重要，它让所有实现了 Iterator 的 T 都自动实现 IteratorExt
    impl<T: ?Sized> IteratorExt for T where T: Iterator {}

    pub struct WindowCount<I> {
        pub(crate) iter: I,
        count: u32,
    }

    impl<I: Iterator> Iterator for WindowCount<I> {
        type Item = Vec<I::Item>;

        fn next(&mut self) -> Option<Self::Item> {
            let data = (0..self.count)
                .filter_map(|_| self.iter.next())
                .collect::<Vec<_>>();
            if data.is_empty() {
                None
            } else {
                Some(data)
            }
        }
    }

    #[test]
    fn main_window_count() {
        let data = vec![1, 2, 3, 4, 5];
        let result = data.iter().window_count(2).collect::<Vec<Vec<_>>>();
        println!("{:?}", result);
    }

    // HashMap 结构有两个 u64 的 RandomState，然后是四个 usize，
    // 分别是 bucket_mask, ctrl, growth_left 和 items
    // 我们 transmute 打印之后，再 transmute 回去
    fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
        let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
        println!(
            "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
            name, arr[2], arr[3], arr[4], arr[5]
        );
        unsafe { std::mem::transmute(arr) }
    }

    #[test]
    fn main_hashmap() {
        let map = HashMap::new();

        let mut map = explain("empty", map);

        map.insert('a', 1);
        let mut map = explain("added 1", map);
        map.insert('b', 2);
        map.insert('c', 3);
        let mut map = explain("added 3", map);
        map.insert('d', 4);
        let mut map = explain("added 4", map);
        map.remove(&'a');
        let mut map = explain("final", map);
        map.shrink_to_fit();
        explain("shrink", map);
    }

    #[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
    struct Name {
        pub name: String,
        pub flags: u32,
    }

    impl Name {
        pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
            Self {
                name: name.as_ref().to_string(),
                flags,
            }
        }
    }

    #[test]
    fn main_bt() {
        let mut map = BTreeMap::new();
        map.insert(Name::new("/etc/password", 0x1), 12);
        map.insert(Name::new("/etc/hosts", 0x1), 4);
        map.insert(Name::new("/home/tchen", 0x0), 28);
        for item in map.iter() {
            println!("{:?}", item);
        }
    }
}
