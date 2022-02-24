#![allow(dead_code, unused)]

use std::process::{Command, Stdio};

const VERSION: &str = "1.0.0";

fn main() {
    println!("Welcome to little_rust, current version: {}", VERSION);
}

// check rustc whether exists
fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}

mod toki {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    // async functions cannot be used for tests
    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8090").await?;

        loop {
            let (mut socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }
}

mod whatever {
    use anyhow::{Context, Result};

    #[test]
    fn t1() {
        if let Err(err) = try_main() {
            eprintln!("Error: {}", err);
            err.chain()
                .skip(1)
                .for_each(|c| eprintln!("because: {}", c));
            std::process::exit(1);
        }
    }

    fn try_main() -> Result<()> {
        anyhow::bail!("generate error")
    }
}
