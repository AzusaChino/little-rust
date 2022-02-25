use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("git")
        .args(&["config", "--get", "user.name"])
        .output()
        .ok()
        .unwrap();
    if output.status.success() {
        println!(
            "your git username is {}",
            String::from_utf8_lossy(&output.stdout).trim().to_owned()
        );
    } else {
        println!("No git user configured");
    }
    // unicode 4 Byte
    let x = '中';
    println!("size is {}", std::mem::size_of_val(&x));
    Ok(())
}
