#[cfg(test)]
mod tests {
    use std::{
        fs::{File, OpenOptions},
        io::{BufRead, BufReader, BufWriter, Cursor, Lines, Read, Seek, SeekFrom, Write},
    };

    #[test]
    fn file() {
        let path = "./test.md";
        read_file(path).expect("fail to read file");
        read_file_str(path).expect("fail to read path");
        write_file(path, "Hello World").expect("fail to write path");
        append_file(path, "Hello Again").expect("fail to append path");
        append_and_read(path, "Greetings").expect("fail to append path");
    }

    fn read_file_str(path: &str) -> anyhow::Result<String> {
        let f = File::open(path)?;
        let mut buf = BufReader::new(f);
        let mut content = String::new();
        buf.read_to_string(&mut content)?;
        Ok(content)
    }

    fn read_file(path: &str) -> anyhow::Result<Lines<BufReader<File>>> {
        let file = File::open(path)?;
        let buf = BufReader::new(file);
        Ok(buf.lines())
    }

    fn write_file(path: &str, content: &str) -> anyhow::Result<()> {
        let f = File::open(path)?;
        let mut write_buf = BufWriter::new(f);
        write_buf.write_all(content.as_bytes())?;
        Ok(())
    }

    fn append_file(path: &str, content: &str) -> anyhow::Result<()> {
        let file = OpenOptions::new().read(true).append(true).open(path)?;
        let mut buf = BufWriter::new(file);
        buf.write_all(content.as_bytes())?;
        Ok(())
    }

    fn append_and_read(path: &str, content: &str) -> anyhow::Result<()> {
        let file = OpenOptions::new().read(true).append(true).open(path)?;
        let mut r_buf = BufReader::new(&file);
        let mut w_buf = BufWriter::new(&file);

        let mut file_content = String::new();
        r_buf.read_to_string(&mut file_content)?;
        println!("before: {}", file_content);

        // using pos to seek
        let pos = r_buf.seek(SeekFrom::Current(0))?;
        w_buf.write_all(content.as_bytes())?;
        w_buf.flush()?;

        r_buf.seek(SeekFrom::Start(pos))?;
        r_buf.read_to_string(&mut file_content)?;

        println!("after {}", file_content);
        Ok(())
    }

    extern crate byteorder;
    use anyhow::Result;
    use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt, BE, LE};
    use walkdir::{DirEntry, WalkDir};

    #[test]
    fn bytes() {
        let binary_nums = vec![2, 3, 12, 8, 5, 0];
        // wrap ibnary collection in a cursor to provide seek functionality
        let mut buf = Cursor::new(binary_nums);
        println!(
            "first byte in binary: {:b}",
            buf.read_u8().expect("fail to read byte")
        );

        println!(
            "first int in binary: {}",
            buf.read_i8().expect("fail to read int")
        );

        buf.write_u8(123).expect("fail to overwrite a byte");

        let _ = buf.position();
        buf.set_position(0);

        buf.seek(SeekFrom::End(0)).expect("fail to seek end");

        // read and write in specific endian
        buf.set_position(0);
        println!(
            "little {}",
            buf.read_u32::<LittleEndian>()
                .expect("fail to read in little")
        );

        buf.set_position(0);
        println!(
            "big {}",
            buf.read_u32::<BigEndian>().expect("fail to read in big")
        );

        buf.seek(SeekFrom::End(0)).expect("fail to seek end");
        buf.write_f32::<LittleEndian>(-33.4)
            .expect("fail to write to end");

        let mut read_buf = [0; 5];
        buf.set_position(0);
        buf.read_u16_into::<LittleEndian>(&mut read_buf)
            .expect("fail to read all bytes");
        println!("{:?}", read_buf);
    }

    fn write_dummy_protocol(path: &str) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let mut w_buf = BufWriter::new(&file);

        let magic = b"DummyProtocol";
        w_buf.write_all(magic)?;

        // indicates endian
        w_buf.write_all(b"LE")?;

        // write padding
        w_buf.write_u32::<LE>(0xDEAD)?;
        w_buf.write_u32::<LE>(0xBEEF)?;

        Ok(())
    }

    fn read_protocol(path: &str) -> Result<Vec<u32>> {
        let file = File::open(path)?;
        let mut r_buf = BufReader::new(&file);

        let mut start = [0u8; 13];
        r_buf.read_exact(&mut start)?;

        if &start != b"DummyProtocol" {
            anyhow::bail!("wrong protocol");
        }
        let mut endian = [0u8; 2];
        r_buf.read_exact(&mut endian)?;

        match &endian {
            b"LE" => read_protocol_payload::<LE, _>(&mut r_buf),
            b"BE" => read_protocol_payload::<BE, _>(&mut r_buf),
            _ => anyhow::bail!("fail to parse endianness"),
        }
    }

    fn read_protocol_payload<E, R>(reader: &mut R) -> Result<Vec<u32>>
    where
        E: ByteOrder,
        R: ReadBytesExt,
    {
        const SIZE_U32: usize = 4;
        let mut payload = Vec::new();

        loop {
            let mut raw_payload = [0; SIZE_U32];
            match reader.read(&mut raw_payload)? {
                0 => return Ok(payload),
                SIZE_U32 => {
                    let as_u32 = raw_payload.as_ref().read_u32::<E>()?;
                    payload.push(as_u32)
                }
                _ => {
                    anyhow::bail!("fail to read any u32")
                }
            }
        }
    }

    #[test]
    fn bin_file() {
        let path = "./bar.bin";
        write_dummy_protocol(path).expect("fail to write dummy");
        let payload = read_protocol(path).expect("fail to read protocol");

        for n in payload {
            println!("0x{:X}", n);
        }
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    fn is_dir(entry: &DirEntry) -> bool {
        entry.file_type().is_dir()
    }

    fn has_file_name(entry: &DirEntry, name: &str) -> bool {
        match entry.file_name().to_str() {
            Some(entry) => entry == name,
            None => false,
        }
    }

    #[test]
    fn fs() {
        for entry in WalkDir::new(".") {
            if let Ok(entry) = entry {
                println!("{}", entry.path().display());
            }
        }

        WalkDir::new("./src/bin")
            .into_iter()
            .filter_entry(|entry| !is_hidden(entry))
            .filter_map(Result::ok)
            .for_each(|entry| {
                println!("{}", entry.file_name().to_string_lossy());
            });

        WalkDir::new("./src")
            .into_iter()
            .filter_entry(|entry| is_dir(entry))
            .filter_map(Result::ok)
            .for_each(|entry| {
                println!("{}", entry.file_name().to_string_lossy());
            });

        let are_any_readonly = WalkDir::new("./src")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| has_file_name(e, "main.rs"))
            .filter_map(|e| e.metadata().ok())
            .any(|e| e.permissions().readonly());
        println!("{}", are_any_readonly);

        let total_size = WalkDir::new("./target")
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(|e| e.metadata().ok())
            .filter(|mt| mt.is_file())
            .fold(0, |acc, m| acc + m.len());
        println!("{}", total_size);
    }

    extern crate glob;

    use glob::{glob, glob_with, MatchOptions};

    #[test]
    fn glb() {
        for entry in glob(".src/*.rs").expect("fail to read glob pattern") {
            match entry {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("fail to read file: {:?}", e),
            }
        }

        let options = MatchOptions {
            case_sensitive: false,
            require_literal_leading_dot: true,
            ..Default::default()
        };

        for entry in glob_with("*Ferris[!_]*", options).expect("fail to read glob pattern") {
            if let Ok(path) = entry {
                println!("{:?}", path.display());
            }
        }
    }
}
