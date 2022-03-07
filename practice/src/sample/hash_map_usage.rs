use std::collections::HashMap;

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

    #[test]
    fn test_process() {
        let mut map = HashMap::<String, String>::new();

        process_or_default(&mut map, String::from("abc"));

        assert!(map.contains_key(&"abc".to_string()));
    }
}
