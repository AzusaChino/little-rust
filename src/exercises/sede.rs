#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs::{File, OpenOptions},
        io::{self, BufRead, BufReader, BufWriter, Read, Write},
    };

    use anyhow::Result;
    use serde::{Deserialize, Serialize};

    extern crate csv;

    #[test]
    fn csv() {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("local.csv")
            .expect("fail to create csv file");
        let w_buf = BufWriter::new(&file);
        let r_buf = BufReader::new(&file);
        write_records(w_buf).expect("fail to write csv");
        read_records(r_buf).expect("fail to read csv");
    }

    fn write_records<W>(writer: W) -> Result<()>
    where
        W: Write,
    {
        let _ = csv::WriterBuilder::new()
            .terminator(csv::Terminator::CRLF)
            .buffer_capacity(1024usize)
            .delimiter(b';')
            .double_quote(false)
            .from_writer(BufWriter::new(File::open(".")?));
        let mut wtr = csv::Writer::from_writer(writer);

        wtr.write_record(&["name", "radius", "distance_from_sun", "gravity"])?;
        wtr.write_record(&["Mercury", "0.38", "0.47", "0.38"])?;
        Ok(())
    }

    fn read_records<R>(reader: R) -> Result<()>
    where
        R: Read,
    {
        let mut rdr = csv::Reader::from_reader(reader);
        for r in rdr.records() {
            let r = r?;
            if let Some(name) = r.get(0) {
                println!("name: {}", name);
            }
        }
        Ok(())
    }

    extern crate serde;

    #[derive(Debug, Deserialize, Serialize)]
    struct Planet {
        name: String,
        radius: f32,
        distance: f32,
        gravity: f32,
    }

    #[test]
    fn sd() {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("local.csv")
            .expect("fail to creat file");
        let buf_w = BufWriter::new(&f);
        let buf_r = BufReader::new(&f);

        write_records_ser(buf_w).expect("fail to write csv");
        read_records_de(buf_r).expect("fail to read csv");
    }

    fn write_records_ser<W>(w: W) -> Result<()>
    where
        W: Write,
    {
        let mut wtr = csv::Writer::from_writer(w);
        wtr.serialize(Planet {
            name: "Venus".to_string(),
            radius: 0.95,
            distance: 0.73,
            gravity: 0.38,
        })?;
        wtr.flush()?;
        Ok(())
    }

    fn read_records_de<R>(r: R) -> Result<()>
    where
        R: Read,
    {
        let mut reader = csv::Reader::from_reader(r);
        for result in reader.deserialize() {
            let p: Planet = result?;
            println!("{:?}", p);
        }
        Ok(())
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Preferences {
        person: String,
        language: Language,
        privacy: Privacy,
    }

    #[derive(Debug, Deserialize, Serialize)]
    enum Language {
        EN,
        CN,
        JP,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Privacy {
        display: String,
        auto_correct: Option<Vec<String>>,
        private: bool,
    }

    #[test]
    fn toml() {
        let preferences = Preferences {
            person: "az".to_owned(),
            language: Language::JP,
            privacy: Privacy {
                display: "az's privacy".to_owned(),
                auto_correct: Some(vec!["en-GB".to_owned(), "en-US".to_owned()]),
                private: true,
            },
        };

        let toml = toml::to_string(&preferences).expect("fail to de toml");
        println!("{}", toml);

        let f = OpenOptions::new()
            .read(true)
            .open("./sample.toml")
            .expect("fail to open file");
        let mut buf_r = BufReader::new(&f);
        let mut s = String::new();
        buf_r.read_to_string(&mut s).expect("fail to read file");
        let p: Preferences = toml::from_str(&s).expect("fail to parse toml");
        println!("{:?}", p);
    }

    #[test]
    fn json() {
        let mut map = HashMap::new();
        let stdin = io::stdin();
        for input in stdin.lock().lines() {
            let input = input.expect("fail to read line");
            let kv: Vec<_> = input.split_whitespace().collect();
            map.insert(kv[0].to_owned(), serde_json::json!(kv[1].to_owned()));
        }

        let json = serde_json::to_string_pretty(&map).expect("fail to prettify");
        println!("{}", json);
    }
}
