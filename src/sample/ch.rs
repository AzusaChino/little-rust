#[cfg(test)]
mod test {

    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn main() {
        let (tx, rx) = channel();
        thread::spawn(move || {
            for i in 0..10 {
                tx.send(i).unwrap();
            }
        });

        while let Ok(i) = rx.recv() {
            println!("received {}", i);
        }
    }
}
