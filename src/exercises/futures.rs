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
}
