#![allow(unused)]

#[cfg(test)]
mod tests {
    use sled::IVec;

    trait KeyGetter {
        fn get_key(&self) -> Vec<u8>;
    }

    #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
    struct UserConfig {
        id: u64,
        name: String,
        age: u8,
        config: Vec<u8>,
        explanation: Option<String>,
        new_name: Option<String>,
    }

    impl KeyGetter for UserConfig {
        fn get_key(&self) -> Vec<u8> {
            self.id.to_string().as_bytes().to_vec()
        }
    }

    #[test]
    fn test_sled_basic_ops() {
        sled_basic_ops().unwrap();
    }

    fn sled_basic_ops() -> anyhow::Result<()> {
        let cfg = sled::Config::new()
            .path("db")
            // .use_compression(true)
            .mode(sled::Mode::HighThroughput);
        let db = cfg.open()?;
        db.insert(b"key", b"v1")?;
        db.compare_and_swap(b"key", Some(b"v1"), Some(b"v2"))??;

        let key = b"k1";
        db.insert(key, vec![0]);
        db.merge(key, vec![1]);
        db.merge(key, vec![2]);

        assert_eq!(db.get(key), Ok(Some(IVec::from(vec![0, 1, 2]))));

        // insert will replace
        db.insert(key, vec![3]);

        // merge will insert if non value exists
        db.remove(key);
        db.merge(key, vec![4]);

        Ok(())
    }

    // [dependencies]
    // sled = "0.32"
    // old_sled = { version = "0.31", package = "sled" }
    fn import_from_old() -> anyhow::Result<()> {
        // let old = old_sled::open("my_old_db")?;
        let old = sled::open("my_old_db")?;
        // may be a different version of sled,
        // the export type is version agnostic.
        let new = sled::open("my_new_db")?;

        let export = old.export();
        new.import(export);

        assert_eq!(old.checksum()?, new.checksum()?);

        Ok(())
    }

    #[test]
    fn test_run_sled_with_obj() {
        run_sled_with_obj().unwrap();
    }

    fn run_sled_with_obj() -> anyhow::Result<()> {
        let db: sled::Db = sled::open("db/userconfig")?;
        let user = UserConfig {
            id: 1,
            name: "user1".to_string(),
            age: 18,
            config: vec![1, 2, 3],
            explanation: None,
            new_name: None,
        };
        let key = user.get_key();
        // let user_bytes = serde_json::to_string(&user)?;
        // db.insert(&key, user_bytes.as_bytes())?;

        let new_user = serde_json::from_slice::<UserConfig>(&db.get(key)?.unwrap())?;
        println!("new_user: {:?}", new_user);

        Ok(())
    }
}
