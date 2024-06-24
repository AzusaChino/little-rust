mod common;
mod demo;
mod deps;
mod exercise;
mod exercises;
mod library;
mod files;
mod mem;
mod sample;
mod ui;
mod whatever;

// local protobuf
pub mod pb;

// custom error
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use crate::MyError;

    #[test]
    fn test() {
        let arg = "invalid";
        if arg != "valid" {
            println!("{:?}", MyError::InvalidArgument("invalid arg".to_owned()));
        } else {
            println!("{:?}", "ok");
        }
    }
}
