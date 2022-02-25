#![allow(unused)]
/*
 A multi-threaded runtime for executing asynchronous code.
 An asynchronous version of the standard library.
 A large ecosystem of libraries.
*/
use anyhow::Result;
use mini_redis::client;
use tokio;

const CONFIG_TOML: &str = "Cargo.toml";

#[tokio::main]
async fn main() -> Result<()> {
    // let mut client = client::connect("127.0.0.1:6379").await?;

    // client.set("hello", "world".into()).await?;

    // let result = client.get("hello").await?;

    println!("Got value from the server; result={:?}", 0);
    Ok(())
}
