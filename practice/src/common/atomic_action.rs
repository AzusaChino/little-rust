#[cfg(test)]
mod tests {

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::{hint, thread};

    fn use_as_lock() {
        // Arc is share_ptr
        let spin_lock = Arc::new(AtomicUsize::new(1));
        let spin_lock_clone = Arc::clone(&spin_lock);

        let thread = thread::spawn(move || {
            spin_lock_clone.store(0, Ordering::SeqCst);
        });

        // wait for the other thread to release the lock
        while spin_lock.load(Ordering::SeqCst) != 0 {
            hint::spin_loop();
        }

        if let Err(panic) = thread.join() {
            println!("Thread had an error: {:?}", panic)
        }
    }

    fn join_self() {
        let builder = thread::Builder::new();

        let join_handler: thread::JoinHandle<_> = builder
            .spawn(|| {
                println!("hello~");
            })
            .unwrap();

        join_handler
            .join()
            .expect("Couldn't join on the associated thread");
    }

    #[test]
    fn test() {
        use_as_lock();
        join_self();
    }
}
