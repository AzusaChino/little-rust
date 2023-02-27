#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::{
        error,
        fmt::Debug,
        fs::{File, OpenOptions},
        io::{self, BufReader, BufWriter, Read, Write},
        num, result,
        sync::RwLock,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[derive(Debug)]
    enum AgeReaderError {
        Io(io::Error),
        Parse(num::ParseIntError),
        Negative(),
    }

    type CustomResult<T> = result::Result<T, AgeReaderError>;

    impl std::fmt::Display for AgeReaderError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                AgeReaderError::Io(ref err) => write!(f, "IO error: {}", err),
                AgeReaderError::Parse(ref err) => write!(f, "Parse error: {}", err),
                AgeReaderError::Negative() => write!(f, "logic error"),
            }
        }
    }

    impl error::Error for AgeReaderError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                AgeReaderError::Io(ref err) => Some(err),
                AgeReaderError::Parse(ref err) => Some(err),
                AgeReaderError::Negative() => None,
            }
        }
    }

    impl From<io::Error> for AgeReaderError {
        fn from(value: io::Error) -> Self {
            AgeReaderError::Io(value)
        }
    }

    impl From<num::ParseIntError> for AgeReaderError {
        fn from(value: num::ParseIntError) -> Self {
            AgeReaderError::Parse(value)
        }
    }

    fn read_age(filename: &str) -> CustomResult<i32> {
        let file = OpenOptions::new().read(true).open(filename)?;
        let mut r_buf = BufReader::new(&file);
        let mut content = String::new();
        r_buf.read_to_string(&mut content)?;
        let age: i32 = content.trim().parse()?;
        if age.is_positive() {
            Ok(age)
        } else {
            Err(AgeReaderError::Negative())
        }
    }

    #[test]
    fn error() {
        const FILENAME: &str = "age.txt";
        let r = read_age(FILENAME);
        match r {
            Ok(num) => println!("{num}"),
            Err(AgeReaderError::Io(err)) => eprintln!("fail to open file {FILENAME} {err}"),
            Err(AgeReaderError::Parse(err)) => eprintln!("fail to parse {err}"),
            Err(AgeReaderError::Negative()) => eprintln!("negative"),
        }
    }

    use log::Level;

    #[test]
    fn log_() {
        // set global log level by using env RUST_LOG
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        log::log!(Level::Debug, "env_logger has been initialized");

        log::info!("the program has started");

        if log::log_enabled!(Level::Debug) {
            log::debug!("expensive : {}", expensive_operation());
        }

        log::error!("something bad");
    }

    fn expensive_operation() -> String {
        log::trace!("start");
        let data = "abc".to_owned();
        std::thread::sleep(std::time::Duration::from_secs(1));
        log::trace!("end");
        data
    }

    struct FileLogger {
        level: log::Level,
        writer: RwLock<BufWriter<File>>,
    }

    impl log::Log for FileLogger {
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            metadata.level() <= self.level
        }

        fn log(&self, record: &log::Record) {
            if self.enabled(record.metadata()) {
                let mut writer = self
                    .writer
                    .write()
                    .expect("fail to unlock log file writer in write mode");
                let now = SystemTime::now();
                let timestamp = now
                    .duration_since(UNIX_EPOCH)
                    .expect("fail to generate timestamp");

                write!(
                    writer,
                    "{} {} at {}: {}\n",
                    record.level(),
                    timestamp.as_secs(),
                    record.target(),
                    record.args()
                )
                .expect("fail to log to file");
            }
            self.flush();
        }

        fn flush(&self) {
            self.writer
                .write()
                .expect("fail to unlock log file writer in write mode")
                .flush()
                .expect("fail to flush content");
        }
    }

    impl FileLogger {
        fn init(level: Level, file_name: &str) -> Result<()> {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_name)?;
            let writer = RwLock::new(BufWriter::new(file));
            let logger = FileLogger { level, writer };
            log::set_max_level(level.to_level_filter());
            log::set_boxed_logger(Box::new(logger))?;

            Ok(())
        }
    }

    #[test]
    fn logger() {
        FileLogger::init(Level::Info, "log.log").expect("fail to init logger");
        log::info!("hah, test");
    }

    struct CustomSmartPointer<D>
    where
        D: Debug,
    {
        data: D,
    }

    impl<D> CustomSmartPointer<D>
    where
        D: Debug,
    {
        fn new(data: D) -> Self {
            Self { data }
        }
    }

    impl<D> Drop for CustomSmartPointer<D>
    where
        D: Debug,
    {
        fn drop(&mut self) {
            println!("dropping csp with data `{:?}`", self.data);
        }
    }

    #[test]
    fn droping() {
        let a = CustomSmartPointer::new("A");
        // not allowed to use
        // a.drop();
        std::mem::drop(a);
        
        let _ = CustomSmartPointer::new("B");
        let _ = CustomSmartPointer::new("C");
        let _ = CustomSmartPointer::new("D");
    }
}
