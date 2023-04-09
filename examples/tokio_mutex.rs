use std::sync::Arc;
#[allow(unused_imports, unused)]
use tokio::{self, sync::{self, Mutex}, runtime::Runtime, time::{self, Duration}};

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(tokio_mutex());

    rt.block_on(std_mutex());

    rt.block_on(cross_await());
}

async fn tokio_mutex(){
    println!("================tokio_mutex================");
    let mutex = Arc::new(sync::Mutex::new(0));

    for i in 0..10 {
        let lock = Arc::clone(&mutex);
        tokio::spawn(async move {
            // 任务的调度顺序是随机的，但是数据加1的操作是依次完成的。
            let mut data = lock.lock().await;
            *data += 1;
            println!("task: {}, data: {}", i, data);
        });
    }

    time::sleep(Duration::from_secs(1)).await;
}

async fn std_mutex(){
    println!("================std_mutex================");
    let mutex = Arc::new(std::sync::Mutex::new(0));

    for i in 0..10 {
        let lock = mutex.clone();
        tokio::spawn(async move {
            let mut data = lock.lock().unwrap();
            *data += 1;
            println!("task: {}, data: {}", i, data);
        });
    }

    time::sleep(Duration::from_secs(1)).await;
}


async fn add_1(mutex: &Mutex<u64>) -> u64{
    let mut lock = mutex.lock().await;
    *lock += 1;
    time::sleep(Duration::from_millis(*lock)).await;
    *lock
}

/**
 * 什么情况下可以选择使用tokio的Mutex？当跨await的时候，可以考虑使用tokio Mutex，
 * 因为这时使用标准库的Mutex将编译错误。当然，也有相应的解决方案。
 * 
 * 什么是跨await？每个await都代表一个异步任务，跨await即表示该异步任务中出现了至少一个子任务。
 * 而每个异步任务都可能会被tokio内部偷到不同的线程上执行，因此跨await时要求其父任务实现Send Trait，
 * 这是因为子任务中可能会引用父任务中的数据。
 */
async fn cross_await(){
    println!("================cross_await================");
    let mutex = Arc::new(Mutex::new(0));

    for i in 0..10 {
        let lock = mutex.clone();
        tokio::spawn(async move {
            let data = add_1(&lock).await;
            println!("task: {}, data: {}", i, data);
        });
    }

    time::sleep(Duration::from_secs(1)).await;
}