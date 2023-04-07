use tokio::sync::watch;

#[tokio::main]
async fn main() {
    // 创建watch通道时，需指定一个初始值存放在通道中
    let (tx, mut rx) = watch::channel("hello");

    // Sender的subscribe()方法可生成新的Receiver
    let mut rx2 = tx.subscribe();

    // Recevier端，通过changed()来等待通道的数据发生变化
    // 通过borrow()引用通道中的数据
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("rx received = {:?}", *rx.borrow());
        }

        while rx2.changed().await.is_ok() {
            println!("rx2 received = {:?}", *rx2.borrow());
        }
    });

    // 向通道中发送数据，实际上是修改通道中的那个数据
    println!("tx send = world");
    tx.send("world").unwrap();
    println!("tx send borrow = {:?}", *tx.borrow());
    println!("tx send_replace = re hello world");
    tx.send_replace("re hello world!");
    println!("tx send_replace borrow = {:?}", *tx.borrow());
}