use async_result::AsyncResult;

const HELLO: &str = "hello";

async fn tokio_wait() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!(
        "read something in thread {:?}",
        std::thread::current().name().unwrap()
    );
    HELLO.to_string()
}

async fn async_std_wait() -> String {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    println!(
        "read something in thread {:?}",
        std::thread::current().name().unwrap()
    );
    HELLO.to_string()
}

fn main() {
    println!(
        "Main thread {:?}",
        std::thread::current().name().unwrap()
    );
    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    async_std::task::block_on(async {
        println!("\nStart running [tokio_wait] in 'async_std'");
        let start = std::time::Instant::now();
        let res = AsyncResult::with(|completer| {
            tokio_rt.spawn(async {
                let res = tokio_wait().await; // tokio_wait() is running in tokio runtime
                completer.complete(res);
            });
        })
        .await
        .unwrap();
        assert_eq!(res, HELLO);
        println!(
            "Running [tokio_wait] in 'async_std' cost {:?}s",
            start.elapsed().as_secs()
        );

        println!("\nStart running [async_std_wait] and [tokio_wait] in 'async_std'");
        let start = std::time::Instant::now();
        let res = AsyncResult::with_async(|completer| async {
            let _ = async_std_wait().await; // async_std_wait() is running in async_std runtime(current main thread)
            tokio_rt.spawn(async {
                let res = tokio_wait().await; // tokio_wait() is running in tokio runtime
                completer.complete(res);
            });
        })
        .await
        .unwrap();
        assert_eq!(res, HELLO);
        println!(
            "Running [async_std_wait] and [tokio_wait] in 'async_std' cost {:?}s",
            start.elapsed().as_secs()
        );

        // split example
        let (completer, fut) = AsyncResult::new_split();
        completer.complete(HELLO.to_string());
        let res = fut.await.unwrap();
        assert_eq!(res, HELLO);
    });
}
