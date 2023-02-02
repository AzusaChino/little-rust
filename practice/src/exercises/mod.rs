pub mod closures;
pub mod common;
pub mod concurrent;
pub mod io;
mod lifecycle;
pub mod pkg;
pub mod req;

mod server {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::{Arc, Mutex};

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
