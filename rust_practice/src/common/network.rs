#[cfg(test)]
mod tests {
    // sync
    use std::io::{Read, Write};
    use std::net::{IpAddr, TcpListener};
    use std::thread;

    // async
    use tokio::net::TcpListener as AsyncTcpListener;

    #[test]
    fn sync_server() {
        let addr = IpAddr::from([127, 0, 0, 1]);
        let listener = TcpListener::bind((addr, 8080)).unwrap();

        loop {
            let (mut stream, addr) = listener.accept().unwrap();
            println!("accept from {}", addr);
            thread::spawn(move || {
                let mut buf = [0; 1024];
                let n = stream.read(&mut buf).unwrap();
                println!("read {} bytes", n);

                let response = b"HTTP/1.1 200 OK\r\n\r\n";
                stream.write(response).unwrap();
                stream.flush().unwrap();
            });
        }
    }

    #[test]
    fn test_async() {
        use tokio::runtime::Runtime;

        let rt = Runtime::new().unwrap();
        rt.block_on(async_server()).unwrap();
    }

    async fn async_server() -> anyhow::Result<()> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let addr = IpAddr::from([127, 0, 0, 1]);
        let listener = AsyncTcpListener::bind((addr, 8080)).await?;

        loop {
            let (mut stream, addr) = listener.accept().await?;
            println!("accept from {}", addr);

            let mut buf = [0; 1024];
            let n = stream.read(&mut buf).await?;
            println!("read {} bytes", n);

            let response = b"HTTP/1.1 200 OK\r\n\r\n";
            stream.write(response).await?;
            stream.flush().await?;
        }
    }
}
