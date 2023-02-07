pub mod closures;
pub mod common;
pub mod concurrent;
pub mod io;
pub mod lifecycle;
pub mod pkg;
pub mod req;
pub mod string;

mod server {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::{Arc, Mutex};

    #[allow(unused)]
    fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = match stream.read(&mut buffer) {
                Ok(n) => n,
                Err(_) => {
                    println!("Error reading from stream");
                    break;
                }
            };
            if bytes_read == 0 {
                println!("Client disconnected, failed to read data");
                break;
            }
            let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("received msg: {}", msg);

            let mut clients = match clients.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    println!("Error acquiring lock on clients");
                    break;
                }
            };

            for client in clients.iter_mut() {
                if let Err(_) = client.write(&buffer[..bytes_read]) {
                    println!("Error writing to stream");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_collect() {
        let _a: Vec<_> = (b'A'..b'z' + 1)
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect();
        println!("{:?}", _a);
    }

    #[test]
    fn test_match() {
        // match guard
        enum Temp {
            C(i32),
            F(i32),
        }
        let t = Temp::C(35);
        let _t = Temp::F(34);
        match t {
            Temp::C(tt) if tt > 30 => println!("{}", tt),
            Temp::C(tt) => println!("{}", tt),
            Temp::F(ff) if ff > 86 => println!("{}", ff),
            Temp::F(ff) => println!("{}", ff),
        }

        fn age() -> u32 {
            15
        }
        match age() {
            0 => println!("oops"),
            // catch arm as variable
            n @ 1..=12 => println!("{}", n),
            n @ 13..=19 => println!("{}", n),
            n => println!("{}", n),
        }

        fn some_number() -> Option<i32> {
            Some(1)
        }

        match some_number() {
            Some(n @ 1) => println!("{}", n),
            Some(n) => println!("{},,", n),
            _ => (),
        }
    }
}
