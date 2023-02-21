#[cfg(test)]
mod tests {
    use std::{
        fs::{File, OpenOptions},
        io::{BufRead, BufReader, BufWriter, Lines, Read, Seek, Write},
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
        let pos = r_buf.seek(std::io::SeekFrom::Current(0))?;
        w_buf.write_all(content.as_bytes())?;
        w_buf.flush()?;

        r_buf.seek(std::io::SeekFrom::Start(pos))?;
        r_buf.read_to_string(&mut file_content)?;

        println!("after {}", file_content);
        Ok(())
    }
}
