#![allow(unused)]
// How to create threads to run multiple pieces of code at the same time
// Message-passing concurrency, where channels send messages between threads
// Shared-state concurrency, where multiple threads have access to some piece of data
// The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

// std::marker
// The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads.
// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.

mod thd {
    // Race conditions, Deadlocks, Bugs
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    fn mn() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(1000));
        }
    }

    // waiting for all threads to finish using join handles
    fn mn_handle() {
        let handle = create_handle(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(1));
        }

        // calling join on the handle blocks the thread currently running until the thread represented by the handle terminates
        handle.join().unwrap();
    }

    fn mn_handle_() {
        let handle: JoinHandle<()> = create_handle(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn create_handle(f: fn()) -> JoinHandle<()> {
        return thread::spawn(move || f());
    }

    fn move_c() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            // println! only needs a reference to v
            println!("there is a vector: {:?}", v);
        });

        // drop(v);

        // block main thread
        handle.join().unwrap();
    }
}

// message passing
mod mp {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    fn ch() {
        let (sender, receiver) = mpsc::sync_channel(1);
        // this returns immediately
        sender.send(1).unwrap();

        thread::spawn(move || {
            // this will block until the previous message has been received
            sender.send(2).unwrap();
        });

        assert_eq!(receiver.recv().unwrap(), 1);
        assert_eq!(receiver.recv().unwrap(), 2);
    }

    fn mm() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("val");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got {}", received);
    }

    // channels and ownership transference
    fn cot() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("Val");
            tx.send(val).unwrap();
            // borrowed after move
            // println!("val is {}", val);
        });

        let r = rx.recv().unwrap();
        println!("Got {}", r);
    }

    // sending multiple values and seeing receiver waiting
    fn rw() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got : {}", received);
        }
    }

    fn trw() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}

// Shared State Concurrency
mod ssc {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // using mutexes to allow access to data from one thread at a time
    fn a() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    // sharing a mutex<T> between multiple threads
    fn sm() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_sm() {
            sm();
        }
    }
}
