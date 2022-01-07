/*
  A multi-threaded runtime for executing asynchronous code.
  An asynchronous version of the standard library.
  A large ecosystem of libraries.
 */
use tokio;

#[tokio::main]
async fn main() {
    let blocking_task = tokio::task::spawn_blocking(|| {
        print_hello();
    });

    blocking_task.await.unwrap();
}


fn print_hello() {
    println!("hello");
}
