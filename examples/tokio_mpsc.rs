use tokio::{ self, sync::mpsc, time::{self, Duration} };
use chrono::Local;

#[tokio::main]
async fn main() {
    mpsc().await;
    reserve().await;
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn mpsc(){
    // tx是Sender端，rx是Receiver端  接收端接收数据时需修改状态，因此声明为mut 
    let (tx, mut rx) = mpsc::channel::<i32>(5);

    tokio::spawn(async move {
        // 通道容量为5，但要发送7个数据，前5个数据会立即发送，发送第6个消息的时候将等待，直到1秒后Receiver开始从通道中消费数据
        for i in 1..=7 {
            // if let Err(_) = tx.send(i).await {}
            if tx.send(i).await.is_err() {
                println!("receiver closed");
                return;
            }
            println!("sended: {}, {}", i, now());
        }
    });

    time::sleep(Duration::from_secs(1)).await;
   // Receiver端则在while循环中不断从通道中取数据。
    while let Some(i) = rx.recv().await {
        println!("received: {}", i);
    }
}

async fn reserve() {
     // 创建容量为1的通道
     let (tx, mut rx) = mpsc::channel(1);
     // 申请并占有唯一的空闲位置
     let permit = tx.reserve().await.unwrap();
     // 唯一的位置已被permit占有，tx.send()无法发送消息
     assert!(tx.try_send(123).is_err());
     // Permit可以通过send()方法向它占有的那个位置发送消息
     permit.send(456);
     // Receiver端接收到消息
     assert_eq!(rx.recv().await.unwrap(), 456);
 
 
     // 创建容量为1的通道
     let (tx, mut rx) = mpsc::channel(1);
     // tx.reserve_owned()会消费掉tx
     let permit = tx.reserve_owned().await.unwrap();
     // 通过permit.send()发送消息，它又返回一个Sender
     let tx = permit.send(456);
     assert_eq!(rx.recv().await.unwrap(), 456);
     //可以继续使用返回的Sender发送消息
     tx.send(789).await.unwrap();
}