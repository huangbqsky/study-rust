#![allow(unused_imports, unused, dead_code)]

use std::sync::Arc;
use tokio::{self, sync::{self, Mutex}, runtime::Runtime, time::{self, Duration}};

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(tokio_mutex());

    rt.block_on(std_mutex());

    rt.block_on(cross_await());
    rt.block_on(cross_await_std_mutex());
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


/**
 * 继续使用标准库的Mutex: 
 *方案1. 可以在子任务await之前，把所有未实现Send的数据都drop掉，保证子任务无法引用父任务中的任何非Send数据。
 */
async fn add_drop_std_mutex(mutex: &std::sync::Mutex<u64>) {
    {
      let mut lock = mutex.lock().unwrap();
      *lock += 1;
    }
    // 子任务，跨await，不引用父任务中的数据
    time::sleep(Duration::from_millis(10)).await;
}

/**
 * 继续使用标准库的Mutex：
 * 方案2：将子任务sleep().await从这个函数中移走。
 */
async fn add_std_mutex(mutex: &std::sync::Mutex<u64>) -> u64 {
    let mut lock = mutex.lock().unwrap();
    *lock += 1;
    *lock
}  // 申请的互斥锁在此被释放

/**
 * tokio的Mutex性能相对较差一些，因此可以不使用tokio Mutex的情况下，尽量不使用它。
 * 可以继续使用标准库的Mutex，但需要做一些调整:
 * 方案1. 可以在子任务await之前，把所有未实现Send的数据都drop掉，保证子任务无法引用父任务中的任何非Send数据。
 * 方案2. 将子任务sleep().await从这个函数中移走。
 * 
 * 这种方案的主要思想是让子任务和父任务不要出现不安全的数据交叉
 * 
 */
async fn cross_await_std_mutex(){
    let mutex = Arc::new(std::sync::Mutex::new(0));

    for i in 0..100 {
        let lock = mutex.clone();
        tokio::spawn(async move {
            let n = add_std_mutex(&lock).await;
            time::sleep(Duration::from_millis(n)).await;
            println!("task: {}, data: {}", i, n);
        });
    }

    time::sleep(Duration::from_secs(1)).await;
    println!("data: {}", mutex.lock().unwrap());
}

