#[cfg(test)]
mod test {
    // multi-producer, single-consumer
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

        while let Ok(r) = rx.recv() {
            println!("received: {}", r);
        }
    }
    #[test]
    fn multi_main() {
        let (tx, rx) = channel();
        for i in 0..10 {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(i).unwrap();
            });
        }
        drop(tx);

        while let Ok(r) = rx.recv() {
            println!("received: {}", r)
        }
    }

    #[test]
    fn sync_ch() {
        use std::sync::mpsc::sync_channel;
        let (tx, rx) = sync_channel(1);
        tx.send(1).unwrap();
        println!("send first arg");
        thread::spawn(move || {
            tx.send(2).unwrap();
            println!("send second arg");
        });

        println!("received first {}", rx.recv().unwrap());
        println!("received second {}", rx.recv().unwrap());
    }
}
