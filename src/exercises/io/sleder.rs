#![allow(unused)]

#[cfg(test)]
mod tests {
    use sled::IVec;

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
}
