mod guessing_game;

#[cfg(test)]
mod tests {
    use io_uring::IoUring;

    #[test]
    fn io_uring_test() {
        io_uring_main().unwrap();
    }

    fn io_uring_main() -> anyhow::Result<()> {
        let mut _ring = IoUring::new(8)?;
        let mut _buf = [0; 13];
        Ok(())
    }
}
