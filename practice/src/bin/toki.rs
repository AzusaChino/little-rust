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

    another_runner();

    Ok(())
}

fn test_receive(mut r: Receiver<i32>) {
    tokio::spawn(async move {
        assert_eq!(r.recv().await.unwrap(), 10);
        assert_eq!(r.recv().await.unwrap(), 20);
    });
}

/**
 * Some actions about operating stuffs
 */
fn another_runner() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|f| f.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
