#[cfg(target_os = "windows")]
fn main() {
    println!("Oops, It's windows")
}

#[cfg(target_os = "linux")]
fn main() {
    try_fork();
}

#[cfg(target_os = "linux")]
fn try_fork() {
    use nix::sys::wait::waitpid;
    use nix::unistd::{fork, write, ForkResult};

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
            // 退出子线程代码块, 不会kill子线程
            unsafe { libc::_exit(0) };
        }
        Err(_) => eprintln!("fork failed"),
    }
}
