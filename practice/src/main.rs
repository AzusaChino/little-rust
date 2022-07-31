use anyhow::Result;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[allow(unused)]
async fn async_sample() {
    println!("I am a async function");
    // call local lib func
    plato::print_hello();
}

// The #[tokio::main] function is a macro.
// It transforms the async fn main() into a synchronous fn main() that initializes a runtime instance and executes the async main function.
#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    tx.send("Bingo").unwrap();

    let res = rx.await;
    println!("Got {:?}", res);

    Ok(())
}

#[allow(unused)]
async fn echo_server() -> io::Result<()> {
    let listener = TcpListener::bind("172.0.0.1:6542").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}

#[allow(unused)]
async fn echo_client() -> io::Result<()> {
    let socket = TcpStream::connect("172.0.0.0:6542").await?;
    let (mut rd, mut wr) = io::split(socket);
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        Ok::<_, io::Error>(())
    });
    let mut buf = vec![0; 128];
    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("Got {:?}", &buf[..n]);
    }
    Ok(())
}
