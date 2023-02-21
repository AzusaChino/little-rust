pub trait Fork {
    /**
     * use specific api to fork a child process
     */
    fn try_fork();
}

#[derive(Debug, Default)]
pub struct Handler;

#[cfg(target_os = "windows")]
mod windows {
    use super::{Fork, Handler};

    impl Fork for Handler {
        fn try_fork() {
            unimplemented!("wait for windows");
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use super::{Fork, Handler};

    impl Fork for Handler {
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
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use super::{Fork, Handler};

    impl Fork for Handler {
        fn try_fork() {
            println!("try fork child-process in macos");
        }
    }
}

fn main() {
    // use different try_fork by current os
    Handler::try_fork();
}
