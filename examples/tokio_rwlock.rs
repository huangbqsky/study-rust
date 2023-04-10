///! tokio RwLock实现的是写锁优先，它的具体规则如下：
///! 每次申请锁时都将等待，申请锁的异步任务被切换，CPU交还给调度器
///! 如果申请的是读锁，并且此时没有写锁存在，则申请成功，对应的任务被唤醒
///! 如果申请的是读锁，但此时有写锁(包括写锁申请)的存在，那么将等待所有的写锁释放(因为写锁总是优先)
///! 如果申请的是写锁，如果此时没有读锁的存在，则申请成功
///! 如果申请的是写锁，但此时有读锁的存在，那么将等待当前正在持有的读锁释放

use std::{sync::Arc, time::Duration};

use tokio::{sync::RwLock, time};

#[tokio::main]
async fn main() {
    read_and_write().await;
    read_and_write_unlock().await;
}

/**
 * 1. 同时多个读锁共存
 * 2. 同时只允许一个写锁存在
 */
async fn read_and_write(){
    let lock = RwLock::new(5);

    // 多个读锁共存
    {
        // read()返回RwLockReadGuard
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5);  // 对Guard解引用，即可得到其内部的值
        assert_eq!(*r2, 5);
    } // 读锁(r1, r2)在此释放

    // 只允许一个写锁存在
    {
        // write()返回RwLockWriteGuard
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    } // 写锁(w)被释放
}

/**
 * 当要使用写锁时，如果要避免死锁，一定要保证同一个任务中的任意两次锁申请之间，前面已经无锁，并且写锁尽早释放。
 */
async fn read_and_write_unlock(){
    let lock = Arc::new(RwLock::new(0));

    let lock1 = lock.clone();
    tokio::spawn(async move {
        let n = lock1.read().await;
        println!("RwLock lock1 n :{}", n);
        drop(n);  // 在申请第二把读锁前，先释放第一把读锁

        time::sleep(Duration::from_secs(2)).await;
        let nn = lock1.read().await;
        println!("RwLock lock1 nn :{}", nn);
        drop(nn);
    });

    time::sleep(Duration::from_secs(1)).await;
    let mut wn = lock.write().await;
    *wn = 2;
    println!("RwLock write :{}", wn);
    // 写锁会自动释放，因此无需手动释放
    // drop(wn);
}