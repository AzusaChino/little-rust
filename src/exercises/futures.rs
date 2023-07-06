#[cfg(test)]
#[allow(unused)]
mod tests {
    use anyhow::Result;
    use chrono::Duration;
    use futures::executor::LocalPool;
    use futures::future::{lazy, ready, Ready};
    use futures::never::Never;
    use futures::prelude::*;

    #[derive(Clone, Copy, Debug)]
    enum Status {
        Loading,
        FetchingData,
        Loaded,
    }

    #[derive(Clone, Copy, Debug)]
    struct Container {
        name: &'static str,
        status: Status,
        ticks: i64,
    }

    impl Container {
        fn new(name: &'static str) -> Self {
            Self {
                name,
                status: Status::Loading,
                ticks: 3,
            }
        }

        fn pull_score(&mut self) -> Ready<Result<u32, Never>> {
            self.status = Status::Loaded;
            std::thread::sleep(
                Duration::seconds(self.ticks)
                    .to_std()
                    .expect("fail to parse chrono"),
            );
            ready(Ok(100))
        }
    }

    impl Future for Container {
        type Output = ();

        fn poll(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            std::task::Poll::Ready(())
        }
    }

    struct Sleep(u64);

    impl Future for Sleep {
        type Output = ();

        fn poll(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            std::task::Poll::Ready(())
        }
    }
    impl Sleep {
        async fn sleep(self) {
            async move { tokio::time::sleep(std::time::Duration::from_secs(self.0)).await };
        }
    }

    const FINISHED: Result<(), Never> = Ok(());

    fn local_util() {
        let mut container = Container::new("acme");
        let mut pool = LocalPool::new();
        let mut exec = pool.spawner();

        let f = lazy(move |_| -> Ready<Result<Container, Never>> {
            container.status = Status::FetchingData;
            ready(Ok(container))
        });

        println!("current status: {:?}", container.status);
    }

    async fn learn_song() -> String {
        future::ready("song".to_owned()).await
    }

    async fn sing_song(song: String) {
        println!("Singing {song}");
    }

    async fn dance() {
        println!("Dancing");
    }

    #[test]
    fn fut() {
        async fn learn_and_sing() {
            let song = learn_song().await;
            sing_song(song).await
        }

        async fn async_main() {
            let (f1, f2) = (learn_and_sing(), dance());
            future::join(f1, f2);
        }
        futures::executor::block_on(async_main());
    }

    // multi-thread read/write
    use anyhow::anyhow;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    use serde_json::{json, Value};
    use std::{
        fs,
        thread::{self, JoinHandle},
    };
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio::sync::Mutex;

    struct MyJoinHandle<T>(JoinHandle<Result<T>>);

    impl<T> MyJoinHandle<T> {
        pub fn thread_wait(self) -> Result<T> {
            self.0.join().map_err(|_| anyhow!("thread panic"))?
        }
    }

    fn thread_read(file: &'static str) -> MyJoinHandle<String> {
        let handle = thread::spawn(move || {
            let s = fs::read_to_string(file)?;
            Ok::<_, anyhow::Error>(s)
        });
        MyJoinHandle(handle)
    }

    fn thread_write(file: &'static str, content: String) -> MyJoinHandle<String> {
        let handle = thread::spawn(move || {
            fs::write(file, &content)?;
            Ok::<_, anyhow::Error>(content)
        });
        MyJoinHandle(handle)
    }

    fn json2yaml(content: &str) -> Result<String> {
        let val: Value = serde_json::from_str(content)?;
        Ok(serde_yaml::to_string(&val)?)
    }

    #[test]
    fn test_thread() {
        let th1 = thread_read("src/exercises/futures.rs");
        let th2 = thread_read("src/exercises/futures.rs");

        let content1 = th1.thread_wait().unwrap();
        let content2 = th2.thread_wait().unwrap();

        let yaml1 = json2yaml(&content1).unwrap();
        let yaml2 = json2yaml(&content2).unwrap();

        let th3 = thread_write("src/exercises/futures.yaml", yaml1);
        let th4 = thread_write("src/exercises/futures.yaml", yaml2);

        let _ = th3.thread_wait().unwrap();
        let _ = th4.thread_wait().unwrap();
    }

    use blake3::{Hash, Hasher};
    use futures::{SinkExt, StreamExt};
    use rayon::prelude::*;
    use tokio::sync::{mpsc, oneshot};
    use tokio_util::codec::{Framed, LinesCodec};

    const PREFIX_ZERO: &[u8] = &[0, 0, 0];

    pub fn pow(s: &str) -> Option<(String, u32)> {
        let hasher = blake3_base_hash(s.as_bytes());
        let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
            let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
            &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
        });
        nonce.map(|n| {
            let hash = blake3_hash(hasher, &n).to_hex().to_string();
            (hash, n)
        })
    }

    fn blake3_hash(mut hasher: Hasher, nonce: &u32) -> Hash {
        hasher.update(&nonce.to_le_bytes()[..]);
        hasher.finalize()
    }

    fn blake3_base_hash(data: &[u8]) -> Hasher {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher
    }

    #[tokio::main]
    async fn main1() -> Result<()> {
        let addr = "0.0.0.0:8081";
        let listener = TcpListener::bind(addr).await?;

        let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

        // thread process
        thread::spawn(move || {
            while let Some((line, reply)) = receiver.blocking_recv() {
                let result = match pow(&line) {
                    Some((hash, nonce)) => format!("{}:{}", hash, nonce),
                    None => "not found".to_string(),
                };
                if let Err(e) = reply.send(result) {
                    println!("send error: {:?}", e);
                }
            }
        });

        loop {
            let (stream, addr) = listener.accept().await?;
            let sender = sender.clone();

            tokio::spawn(async move {
                let framed = Framed::new(stream, LinesCodec::new());
                let (mut writer, mut reader) = framed.split();
                while let Some(Ok(line)) = reader.next().await {
                    let (reply, reply_receiver) = oneshot::channel();
                    sender.send((line, reply)).unwrap();

                    if let Ok(v) = reply_receiver.await {
                        if let Err(e) = writer.send(v).await {
                            println!("send error: {:?}", e);
                        }
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
        }
    }

    // 简单的 async 方法 @see Future
    async fn write_hello_write_async(name: &'static str) -> Result<()> {
        let mut file = File::create("hello.txt").await?;
        file.write_all(format!("hello, {}", name).as_bytes())
            .await?;

        Ok(())
    }

    // 手动实现的状态转换 类 async 方法
    enum WriteHelloFile {
        Init(String),
        AwaitingCreate(Box<dyn Future<Output = Result<File>>>),
        AwaitingWriteAll(Box<dyn Future<Output = Result<()>>>),
        Done,
    }

    impl WriteHelloFile {
        pub fn new(name: impl Into<String>) -> Self {
            Self::Init(name.into())
        }
    }
}
