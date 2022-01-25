use nix::sys::wait::waitpid;
use nix::unistd::{fork, write, ForkResult};

fn main() {
    try_fork();
}

fn try_fork() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
            unsafe { libc::_exit(0) };
        }
        Err(_) => eprintln!("fork failed"),
    }
}
