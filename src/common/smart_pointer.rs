//! 在 Rust 中，凡是需要做资源回收的数据结构，且实现了Deref/DerefMut/Drop，都是智能指针。

#[allow(dead_code)]
#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::{
        borrow::{Borrow, Cow},
        ops::Deref,
    };

    use serde::Deserialize;

    #[derive(Debug, Default)]
    struct UserSample {
        username: String,
        password: String,
    }

    #[test]
    fn boxi() {
        let mut boxed_obj = Box::new(UserSample::default());
        boxed_obj.username = String::from("username");
        boxed_obj.password = "password".to_string();
        drop(boxed_obj)
    }

    fn remove_whitespace(s: &str) -> String {
        s.to_string().replace(' ', "")
    }

    fn remove_ws_cow(s: &str) -> Cow<str> {
        if s.contains(' ') {
            Cow::Owned(s.to_string().replace(' ', ""))
        } else {
            Cow::Borrowed(s)
        }
    }
    // 包裹一个只读借用，但如果调用者需要所有权或者需要修改内容，那么它会 clone 借用的数据。
    #[test]
    fn call_cow() {
        // 如果参数不包含空格，会 Copy出一份多余的内存
        let v1 = remove_whitespace("HelloWorld");
        let v2 = remove_ws_cow("Hello World");

        println!("{}, {}", v1, v2);
    }

    struct User<'a> {
        first_name: Cow<'a, str>,
        last_name: Cow<'a, str>,
    }

    impl<'a> User<'a> {
        pub fn new_owned(first_name: String, last_name: String) -> User<'static> {
            User {
                first_name: Cow::Owned(first_name),
                last_name: Cow::Owned(last_name),
            }
        }

        pub fn new_borrowed(first_name: &'a str, last_name: &'a str) -> Self {
            Self {
                first_name: Cow::Borrowed(first_name),
                last_name: Cow::Borrowed(last_name),
            }
        }

        pub fn first_name(&self) -> &str {
            &self.first_name
        }
        pub fn last_name(&self) -> &str {
            &self.last_name
        }
    }

    // The true power of Cow comes with to_mut method. If the Cow is owned,
    // it simply returns the pointer to the underlying data,
    // however if it is borrowed, the data is first cloned to the owned from.
    #[test]
    fn main_cow() {
        // Static lifetime as it owns the data
        let user: User<'static> = User::new_owned("James".to_owned(), "Bond".to_owned());
        println!("Name: {} {}", user.first_name, user.last_name);

        // Static lifetime as it borrows 'static data
        let user: User<'static> = User::new_borrowed("Felix", "Leiter");
        println!("Name: {} {}", user.first_name, user.last_name);

        let first_name = "Eve".to_owned();
        let last_name = "Moneypenny".to_owned();

        // Non-static lifetime as it borrows the data
        let user = User::new_borrowed(&first_name, &last_name);
        println!("Name: {} {}", user.first_name, user.last_name);
    }

    struct LazyBuffer<'a> {
        data: Cow<'a, [u8]>,
    }

    impl<'a> LazyBuffer<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            Self {
                data: Cow::Borrowed(data),
            }
        }

        pub fn data(&self) -> &[u8] {
            &self.data
        }

        pub fn append(&mut self, data: &[u8]) {
            self.data.to_mut().extend(data)
        }
    }

    #[test]
    fn main_lazy() {
        let data = vec![0u8; 10];

        // No memory copied yet
        let mut buffer = LazyBuffer::new(&data);
        println!("{:?}", buffer.data());

        // The data is cloned
        buffer.append(&[1, 2, 3]);
        println!("{:?}", buffer.data());

        // The data is not cloned on further attempts
        let new_data = vec![4, 5, 6];
        buffer.append(&new_data);
        println!("{:?}", buffer.data());
    }

    #[derive(Debug)]
    struct MyString {
        data: String,
    }

    #[derive(Debug)]
    #[repr(transparent)]
    struct MyStr {
        data: str,
    }

    impl Borrow<MyStr> for MyString {
        fn borrow(&self) -> &MyStr {
            unsafe { &*(self.data.as_str() as *const str as *const MyStr) }
        }
    }

    impl ToOwned for MyStr {
        type Owned = MyString;

        fn to_owned(&self) -> MyString {
            MyString {
                data: self.data.to_owned(),
            }
        }
    }

    impl Deref for MyString {
        type Target = MyStr;

        fn deref(&self) -> &Self::Target {
            self.borrow()
        }
    }

    #[test]
    fn main_owned_obj() {
        let data = MyString {
            data: "Hello world".to_owned(),
        };

        let borrowed_cow: Cow<'_, MyStr> = Cow::Borrowed(&data);
        println!("{:?}", borrowed_cow);

        let owned_cow: Cow<'_, MyStr> = Cow::Owned(data);
        println!("{:?}", owned_cow);
    }

    #[derive(Debug, Deserialize)]
    struct UserObj<'a> {
        #[serde(borrow)]
        name: Cow<'a, str>,
        age: u8,
    }

    #[test]
    fn main_serde() {
        let input = r#"{ "name": "Tyr", "age": 18 }"#;
        let user: UserObj = serde_json::from_str(input).unwrap();
        match user.name {
            Cow::Borrowed(x) => println!("borrowed {}", x),
            Cow::Owned(x) => println!("owned {}", x),
        }
    }

    // mutex_guard

    // lazy_static 宏可以生成复杂的 static 对象
    lazy_static! {
    // 一般情况下 Mutex 和 Arc 一起在多线程环境下提供对共享内存的使用
    // 如果你把 Mutex 声明成 static，其生命周期是静态的，不需要 Arc
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> =
    Mutex::new(HashMap::new());
    }

    #[test]
    fn main_mutex_guard() {
        let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
            Arc::new(Mutex::new(HashMap::new()));
        for _ in 0..32 {
            let m = metrics.clone();
            thread::spawn(move || {
                let mut g = m.lock().unwrap();
                // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
                let data = &mut *g;
                // Cow 实现了很多数据结构的 From trait，
                // 所以我们可以用 "hello".into() 生成 Cow
                let entry = data.entry("hello".into()).or_insert(0);
                *entry += 1;
                // MutexGuard 被 Drop，锁被释放
            });
        }

        thread::sleep(Duration::from_secs(1));

        println!("metrics: {:?}", metrics.lock().unwrap());
    }

    fn print_slice<T: std::fmt::Debug>(s: &[T]) {
        println!("{:?}", s);
    }

    fn print_slice_ref<T, U>(s: T)
    where
        T: AsRef<U>,
        U: std::fmt::Debug + ?Sized,
    {
        println!("{:?}", s.as_ref());
    }

    #[test]
    fn main_slice() {
        let v = vec![1, 2, 3, 4];
        // Vec 实现了 Deref，&Vec<T> 会被自动解引用为 &[T]，符合接口定义
        print_slice(&v);
        // 直接是 &[T]，符合接口定义
        print_slice(&v[..]);
        // &Vec<T> 支持 AsRef<[T]>
        // print_slice_ref(&v);
        // &[T] 支持 AsRef<[T]>
        print_slice_ref(&v[..]);
        // Vec<T> 也支持 AsRef<[T]>
        // print_slice_ref(v);
        let arr = [1, 2, 3, 4];
        // 数组虽没有实现 Deref，但它的解引用就是 &[T]
        print_slice(&arr);
        print_slice(&arr[..]);
        print_slice_ref(&arr);
        print_slice_ref(&arr[..]);
        print_slice_ref(arr);
    }

    #[test]
    fn main_vec_iter() {
        let r = vec![1, 2, 3, 4]
            .iter()
            .map(|v| v * v)
            .filter(|v| *v < 16)
            .take(1)
            // 直到运行到 collect 时才真正开始执行，之前的部分不过是在不断地生成新的结构
            .collect::<Vec<i32>>();
        println!("{:?}", r);
    }

    use itertools::Itertools;

    #[test]
    fn main_itertools() {
        let err_str = "bad result";
        let input = vec![Ok(21), Err(err_str), Ok(8)];

        let it = input
            .into_iter()
            .filter_map_ok(|i| if i > 10 { Some(i * 2) } else { None });

        // 结果应该是：vec![Ok(42), Err(err_str)]
        println!("{:?}", it.collect::<Vec<_>>());
    }

    use std::iter::FromIterator;

    #[test]
    fn main_iter_str() {
        let arr = ['h', 'e', 'l', 'l', 'o'];
        let vec = vec!['h', 'e', 'l', 'l', 'o'];
        let s = String::from("hello");

        let s1 = &arr[1..3];
        let s2 = &vec[1..3];
        let s3 = &s[1..3];

        println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);
        // &[char] 和 &[char] 是否相等取决于长度和内容是否相等
        assert_eq!(s1, s2);
        // &[char] 和 &str 不能直接对比，我们把 s3 变成 Vec<char>
        assert_eq!(s2, s3.chars().collect::<Vec<_>>());
        // &[char] 可以通过迭代器转换成 String，String 和 &str 可以直接对比
        assert_eq!(String::from_iter(s2), s3);
    }
}
