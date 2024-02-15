#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;

    use dashmap::DashMap;

    #[test]
    fn test_dashmap() {
        let dm = DashMap::new();
        dm.insert("macro", "ok");

        let s = RandomState::new();
        let dm_with_hasher =
            DashMap::<u32, i32>::with_capacity_and_hasher_and_shard_amount(2, s, 32);

        dm_with_hasher.insert(2, 3);
        dm_with_hasher.insert(3, 4);
    }
}
