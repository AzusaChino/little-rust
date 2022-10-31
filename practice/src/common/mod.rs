mod atomic_action;
mod copy_clone;
mod enums;
mod extender;
mod path_ruler;

#[cfg(test)]
mod test_reader {
    use std::fs::File;
    use std::io::{BufReader, Read, Result};

    struct MyReader<R> {
        reader: R,
        buf: String,
    }

    impl<R> MyReader<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                buf: String::with_capacity(64),
            }
        }
    }

    impl<R> MyReader<R>
    where
        R: Read,
    {
        pub fn process(&mut self) -> Result<usize> {
            self.reader.read_to_string(&mut self.buf)
        }
    }

    #[test]
    fn main() {
        let f = File::open("./mod.rs").unwrap();
        let mut reader = MyReader::new(BufReader::new(f));

        let size = reader.process().unwrap();
        println!("total size read: {}", size);
    }
}

#[cfg(test)]
mod test_writer {

    use std::fs::File;
    use std::io::BufWriter;
    use std::net::TcpStream;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyWriter<W> {
        writer: W,
    }

    impl MyWriter<BufWriter<TcpStream>> {
        pub fn new(addr: &str) -> Self {
            let stream = TcpStream::connect(addr).unwrap();
            Self {
                writer: BufWriter::new(stream),
            }
        }
    }

    impl MyWriter<File> {
        pub fn new(addr: &str) -> Self {
            let file = File::open(addr).unwrap();
            Self { writer: file }
        }
    }

    impl<W> MyWriter<W> {
        pub fn write(&self, msg: &str) {
            println!("msg: {:?}", msg)
        }
    }

    #[test]
    fn main() {
        let writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
        writer.write("hello world!");

        let writer = MyWriter::<File>::new("/etc/hosts");
        writer.write("127.0.0.1 localhost");
    }
}

#[cfg(test)]
mod test_vec {

    use rand::Rng;

    #[test]
    fn main() {
        let mut data: Vec<*const [u8]> = Vec::new();

        for _i in 0..5 {
            let mut num: Vec<u8> = Vec::new();
            for _j in 0..16 {
                let rand_num: u8 = rand::thread_rng().gen();
                num.push(rand_num);
            }
            println!("num({:p}) is : {:?}", &*num, num);
            let boxed = num.into_boxed_slice();
            data.push(Box::into_raw(boxed) as _);
        }
        println!("data is: {:?}", data);
    }
}

#[cfg(test)]
mod test_trait {
    use regex::Regex;
    use std::fmt;
    use std::io::Write;
    use std::str::FromStr;

    pub trait Parse {
        type Error;
        fn parse(s: &str) -> Result<Self, Self::Error>
        where
            Self: Sized;
    }

    impl<T> Parse for T
    where
        T: FromStr + Default,
    {
        type Error = String;
        fn parse(s: &str) -> Result<Self, Self::Error> {
            let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
            if let Some(captures) = re.captures(s) {
                // 当出错时我们返回 Err(String)
                captures
                    .get(0)
                    .map_or(Err("failed to capture".to_string()), |s| {
                        s.as_str()
                            .parse()
                            .map_err(|_err| "failed to parse captured string".to_string())
                    })
            } else {
                Err("failed to parse string".to_string())
            }
        }
    }

    struct BufBuilder {
        buf: Vec<u8>,
    }

    impl BufBuilder {
        pub fn new() -> Self {
            Self {
                buf: Vec::with_capacity(1024),
            }
        }
    }

    impl fmt::Debug for BufBuilder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", String::from_utf8_lossy(&self.buf))
        }
    }

    impl Write for BufBuilder {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn main() {
        let mut buf = BufBuilder::new();
        buf.write_all(b"HelloWorld").unwrap();
        println!("{:?}", buf);
    }

    #[test]
    fn parse_should_work() {
        assert_eq!(u32::parse("123abcd"), Ok(123));
        assert_eq!(
            u32::parse("123.45abcd"),
            Err("failed to parse captured string".into())
        );
        assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
        assert!(f64::parse("abcd").is_err());
    }
}

#[cfg(test)]
mod test_traitor {
    struct SentenceIter<'a> {
        s: &'a mut &'a str,
        delimiter: char,
    }
    impl<'a> SentenceIter<'a> {
        pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
            Self { s, delimiter }
        }
    }
    impl<'a> Iterator for SentenceIter<'a> {
        type Item = &'a str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.s.is_empty() {
                return None;
            }
            match self.s.find(self.delimiter) {
                Some(pos) => {
                    let len = self.delimiter.len_utf8();
                    let s = &self.s[..pos + len];
                    let suffix = &self.s[pos + len..];
                    *self.s = suffix;
                    Some(s.trim())
                }
                None => {
                    let s = (*self.s).trim();
                    *self.s = "";
                    if s.len() == 0 {
                        None
                    } else {
                        Some(s)
                    }
                }
            }
        }
    }

    #[test]
    fn it_works() {
        let mut s = "This is the 1st sentence. This is the 2nd sentence.";
        let mut iter = SentenceIter::new(&mut s, '.');
        assert_eq!(iter.next(), Some("This is the 1st sentence."));
        assert_eq!(iter.next(), Some("This is the 2nd sentence."));
        assert_eq!(iter.next(), None);
    }
}
