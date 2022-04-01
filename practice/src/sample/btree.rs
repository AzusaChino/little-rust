#[cfg(test)]
mod test {

    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let mut map = BTreeMap::new();
        map.insert(3, "a");
        map.insert(5, "b");
        map.insert(8, "c");

        // query in range
        for (k, v) in map.range(2..6) {
            println!("{}: {}", k, v);
        }
    }
}
