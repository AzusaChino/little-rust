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
