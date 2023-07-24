#![allow(unused)]

#[cfg(test)]
mod tests {

    #[test]
    fn test_mpsc() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        });

        for x in rx.iter() {
            println!("Got: {}", x);
        }
    }
}
