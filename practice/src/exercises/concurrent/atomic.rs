use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn _1() {
    let spin_lock = Arc::new(AtomicUsize::new(1));
    let spin_lock_clone = spin_lock.clone();

    let thread = thread::spawn(move || {
        spin_lock_clone.store(0, Ordering::SeqCst);
    });

    while spin_lock.load(Ordering::SeqCst) != 0 {}

    if let Err(panic) = thread.join() {
        eprintln!("error: {:?}", panic)
    }

    static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

    let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("live threads: {}", old_thread_count);
}