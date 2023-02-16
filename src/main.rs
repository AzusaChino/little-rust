use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    order: String,
}

// The #[tokio::main] function is a macro.
// It transforms the async fn main() into a synchronous fn main() that initializes a runtime instance and executes the async main function.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("your order is {}", args.order);

    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send("msg") {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tokio::{
        io::{self, AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
    };
    #[test]
    fn test() -> Result<()> {
        let r = tokio::runtime::Builder::new_multi_thread().build()?;
        r.spawn(echo_server());
        r.spawn(echo_client());
        Ok(())
    }

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
}
