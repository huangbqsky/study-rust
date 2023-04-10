#![allow(dead_code, unused)]

use tokio::{sync::Notify, time};
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() {
    notify_once().await;
    notify_waiters().await;
}

// Notify提供了一种简单的通知唤醒功能，它类似于只有一个信号灯的信号量。
async fn notify_once(){
    // 创建Notify实例，Notify实例初始时没有permit位，permit可认为是执行权
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    tokio::spawn(async move {
        // 每当调用notified().await时，将判断此时是否有执行权，
        // 如果有，则可直接执行，否则将进入等待。
        // 因此，初始化之后立即调用notified().await将会等待。
        notify2.notified().await;
        println!("received notification");
    });
    println!("just now sending notification");
    // 每当调用notify_one()时，将产生一个执行权，但多次调用也最多只有一个执行权。
    // 因此，调用notify_one()之后再调用notified().await则并无需等待。
    notify.notify_one();
    time::sleep(Duration::from_secs(1)).await;
    println!("sleep 1 seconds sending notification");
    // 多次调用notify_one也最多只有一个执行权
    notify.notify_one();
}

/**
 * Notify还有一个notify_waiters()方法，它不会释放执行权，但是它会一次性唤醒所有正在等待的等候者。
 * 严格来说，是让当前已经注册的等候者(即已经调用notified()，但是还未await)在下次等待的时候，可以直接通过。
 */
async fn notify_waiters(){
    println!("--------notify_waiters---------");
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    // 注册两个等候者
    let notified1 = notify.notified();
    let notified2 = notify.notified();

    let handle = tokio::spawn(async move {
        println!("sending notifications");
        // 会一次性唤醒所有正在等待的等候者
        notify2.notify_waiters();
        println!("notify_waiters after");
    });

    println!("2 notified await");
    // 两个等候者的await都会直接通过
    notified1.await;
    notified2.await;
    println!("received notifications");
}