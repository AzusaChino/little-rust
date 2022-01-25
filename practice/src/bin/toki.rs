use anyhow::Result;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;

#[tokio::main]
async fn main() -> Result<()> {
    let (sender, receiver1) = broadcast::channel(16);
    let re2 = sender.subscribe();

    test_receive(receiver1);
    test_receive(re2);

    sender.send(10).unwrap();
    sender.send(20).unwrap();

    Ok(())
}

fn test_receive(mut r: Receiver<i32>) {
    tokio::spawn(async move {
        assert_eq!(r.recv().await.unwrap(), 10);
        assert_eq!(r.recv().await.unwrap(), 20);
    });
}
