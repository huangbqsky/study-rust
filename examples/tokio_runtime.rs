use std::thread;
use chrono::Local;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt = Runtime::new().unwrap();

    rt.spawn_blocking(|| {
      thread::sleep(std::time::Duration::from_secs(5));
      println!("blocking thread task over: {}", now());
    });
    
    let _guard = rt.enter();
    rt.spawn(async {
      time::sleep(time::Duration::from_secs(3)).await;
      println!("worker thread task over 1: {}", now());
    });

    rt.spawn(async {
      std::thread::sleep(std::time::Duration::from_secs(4));
      println!("worker thread task over 2: {}", now());
    });
    
    // 先让所有任务运行起来
    std::thread::sleep(std::time::Duration::from_millis(3));

    // 1秒后强行关闭Runtime
    rt.shutdown_timeout(std::time::Duration::from_secs(1));
    println!("runtime droped: {}", now());
}