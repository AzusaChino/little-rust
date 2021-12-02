use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct JobStatus {
    jobs_completed: u32,
}

// fn main() {
//     // Atomic Object 线程安全的引用计数指针
//     let status = Arc::new(JobStatus { jobs_completed: 0 });
//     let status_shared = status.clone();
//
//     // move converts any variables captured by reference or mutable reference to variables captured by value.
//     thread::spawn(move || {
//         for _ in 0..10 {
//             thread::sleep(Duration::from_millis(250));
//             status_shared.jobs_completed += 1;
//         }
//     });
//     while status.jobs_completed < 10 {
//         println!("waiting... ");
//         thread::sleep(Duration::from_millis(500));
//     }
// }
