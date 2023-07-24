mod guessing_game;
mod sleder;

#[cfg(test)]
mod tests {
    use io_uring::{opcode, types, IoUring};
    use std::fs;
    use std::os::unix::io::AsRawFd;

    #[test]
    fn io_uring_test() {
        io_uring_main().unwrap();
    }

    // io-uring requires minimum kernel version 5.1
    fn io_uring_main() -> anyhow::Result<()> {
        let mut ring = IoUring::new(8)?;
        // Open the file "README.md"
        let fd = fs::File::open("README.md")?;
        let mut buf = vec![0; 1024];

        let read_e = opcode::Read::new(types::Fd(fd.as_raw_fd()), buf.as_mut_ptr(), buf.len() as _)
            .build()
            // why 0x42?
            .user_data(0x42);

        // Push the read opeartion into the submission queue
        unsafe {
            ring.submission().push(&read_e).expect("queue is full");
        }

        // Submit the read operation and wait for it to complete
        ring.submit_and_wait(1)?;

        // Get the result of the read operation
        let cqe = ring.completion().next().expect("queue is empty");
        assert_eq!(cqe.user_data(), 0x42);
        assert!(cqe.result() >= 0, "read error: {}", cqe.result());

        Ok(())
    }
}
