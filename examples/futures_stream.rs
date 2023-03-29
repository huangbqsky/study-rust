use futures::channel::mpsc;
use futures::{executor::block_on, SinkExt, StreamExt};

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    println!("tx: Send 1, 2");
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
    // 因此还需要使用`.await`来获取具体的值
    let next_one = rx.next().await;
    let next_two = rx.next().await;
    let next_none = rx.next().await;
    println!("rx: {:?} -> {:?} -> {:?}",next_one, next_two, next_none);

    assert_eq!(Some(1), next_one);
    assert_eq!(Some(2), next_two);
    assert_eq!(None, next_none);
}

fn main() {
    block_on(send_recv());   
}