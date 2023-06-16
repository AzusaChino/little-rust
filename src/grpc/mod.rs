include!(concat!(env!("OUT_DIR"), "/hello.rs"));

pub fn _hello() {
    let _hr = HelloRequest {
        name: "hello".to_string(),
        message: "world".to_string(),
    };
}

mod prost {
    use anyhow::Result;
    use prost::Message;

    #[derive(Message)]
    struct HelloRequest {
        #[prost(string, tag = "1")]
        name: String,
        #[prost(string, tag = "2")]
        message: String,

        #[prost(string, optional, tag = "3")]
        extra: Option<String>,
    }

    impl HelloRequest {
        pub fn new() -> Self {
            Self {
                name: "".to_string(),
                message: "".to_string(),
                extra: None,
            }
        }
    }

    #[test]
    fn run() {
        run_test().unwrap();
    }

    fn run_test() -> Result<()> {
        let hr = HelloRequest::new();
        let mut vec = Vec::new();
        hr.encode(&mut vec)?;

        let hr2 = HelloRequest::decode(&vec[..])?;
        println!("{:?}", hr2);
        Ok(())
    }
}
