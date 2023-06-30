#![allow(unused)]

fn basic_usage() {
    // String impl Extend<&'a char>
    let mut message = String::from("the first three letters are: ");

    message.extend(&['a', 'b', 'c']);
    assert_eq!("abc", &message[29..32]);
}

#[derive(Debug)]
struct MyCollection(Vec<i32>);

impl MyCollection {
    fn new() -> Self {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

impl Extend<i32> for MyCollection {
    fn extend<T: IntoIterator<Item = i32>>(&mut self, iter: T) {
        for elem in iter {
            self.add(elem);
        }
    }
}

mod test {
    use super::MyCollection;

    fn test_my_collection() {
        let mut c = MyCollection::new();
        c.add(5);
        c.add(6);
        c.add(7);

        c.extend(vec![1, 2, 3]);

        assert_eq!("MyCollection([5, 6, 7, 1, 2, 3])", format!("{:?}", c));
    }
}

use std::{fmt, mem::size_of, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiniString")
            .field("len", &self.len)
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Inline(ref s) => s.deref(),
            Self::Standard(ref s) => s.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        if s.len() <= MINI_STRING_MAX_LEN {
            Self::Inline(MiniString::new(s))
        } else {
            Self::Standard(s.to_owned())
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[test]
fn test_my_string() {
    let len1 = size_of::<MyString>();
    let len2 = size_of::<MiniString>();
    let len3 = size_of::<String>();
    println!("len1: {}, len2: {}, len3: {}", len1, len2, len3);

    let s1: MyString = "hello world".into();
    let s2: MyString = "hello world hello world hello world".into();
    println!("s1: {:?}, s2: {:?}", s1, s2);

    println!(
        "s1: {}({} bytes, {} chars), s2: {}({}bytes, {}chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );
}
