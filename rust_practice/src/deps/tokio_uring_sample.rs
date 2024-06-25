use crossterm::style::Stylize;
use std::path::Path;

pub async fn read_file(name: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = tokio_uring::fs::File::open(name).await?;
    let buf_mv = vec![0; 4096];

    // Read some data, the buffer is passed by ownership and submitted
    // to the kernel. When the operation completes, we get the buffer
    // back.
    let (result, buf_from_kernel) = file.read_at(buf_mv, 0).await;
    let bytes_read = result?;

    println!(
        "{}",
        format!("Read {} bytes", bytes_read)
            .yellow()
            .underlined()
            .bold()
    );

    println!(
        "{}\n{}",
        "Data (bytes):".yellow().bold().underlined(),
        format!("{:?}", &buf_from_kernel[..bytes_read])
            .blue()
            .bold()
    );

    println!(
        "{}\n{}",
        "Data (string):".yellow().bold().underlined(),
        String::from_utf8_lossy(&buf_from_kernel[..bytes_read])
            .cyan()
            .bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::read_file;
    #[test]
    fn test_read_file() {
        tokio_uring::start(read_file("Cargo.toml")).expect("err");
    }
}
