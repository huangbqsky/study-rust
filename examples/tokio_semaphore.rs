///! 信号量可以保证在某一时刻最多运行指定数量的并发任务。信号量通常用来提供类似于限量的功能
use chrono::Local;
use std::sync::Arc;
use tokio::{ self, runtime::Runtime, sync::Semaphore, time::{self, Duration}};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // 只有3个信号灯的信号量， 意味着同时最多并发执行3个任务
        let semaphore = Arc::new(Semaphore::new(3));

        // 5个并发任务，每个任务执行前都先获取信号灯
        // 因此，同一时刻最多只有3个任务进行并发
        for i in 1..=5 {
            let semaphore = semaphore.clone();
            tokio::spawn(async move {
                // 获取一个信号灯，如果信号量已经被关闭，则返回错误AcquireError
                let _permit = semaphore.acquire().await.unwrap();
                println!("{}, {}", i, now());
                time::sleep(Duration::from_secs(1)).await;
            });
        }

        time::sleep(Duration::from_secs(3)).await;
    });
}