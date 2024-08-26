#![allow(unused)]
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

struct Person {
    first_name: String,
    last_name: String,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first_name.hash(state);
        self.last_name.hash(state);
    }
}

pub fn process_or_default(map: &mut HashMap<String, String>, key: String) {
    match map.get_mut(&key) {
        Some(value) => println!("{}", value),
        None => {
            map.insert(key, String::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::process_or_default;
    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        fn new(first: &str, last: &str) -> Self {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }
    }

    #[test]
    fn test_process() {
        let mut map = HashMap::<String, String>::new();

        process_or_default(&mut map, String::from("abc"));

        assert!(map.contains_key(&"abc".to_string()));
    }

    #[test]
    fn use_map() {
        let mut book = HashMap::new();
        book.insert(Person::new("John", "Smith"), "521-9876");
        book.insert(Person::new("Will", "Smith"), "521-9876");
        book.insert(Person::new("Wiliam", "Smith"), "521-9876");

        let p = Person::new("Will", "Smith");

        if let Some(phone) = book.get(&p) {
            println!("Phone number found: {}", phone);
        }

        book.remove(&p);

        println!("Find key: {}", book.contains_key(&p));
    }
}
