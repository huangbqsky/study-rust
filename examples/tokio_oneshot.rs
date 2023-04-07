use tokio::{self, sync};

#[tokio::main]
async fn main() {
    one_shot().await;
    select().await;
}

async fn one_shot() {
    // 创建一个oneshot::channel() 通道
    let (tx, rx) = sync::oneshot::channel();

    tokio::spawn(async move {
        if tx.send(33).is_err() {
            println!("receiver dropped");
        }
    });

    match rx.await {
        Ok(value) => println!("received: {:?}", value),
        Err(_) => println!("sender dropped"),
    };
}

async fn select(){
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));

    // 注意mut
    let (tx, mut rx) = sync::oneshot::channel();

    tokio::spawn(async move {
        if tx.send(33).is_err() {
            println!("receiver dropped");
        }
    });

    loop {
        // 注意，select!中无需await，因为select!会自动轮询推进每一个分支的任务进度
        tokio::select! {
            _ = interval.tick() => println!("Another 100ms"),
            msg = &mut rx => {
                println!("Got message: {}", msg.unwrap());
                break;
            }
        }
    }
}
