///! Barrier是一种让多个并发任务在某种程度上保持进度同步的手段。
/// 让并发任务的进度按批次进行同步，一批任务都同步后才放行一批，否则一直等待
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Barrier;
use tokio::{
    self,
    time::{self, Duration},
};
fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(barrier_vec());
    rt.block_on(barrier());
}
/**
 * 使用屏障时，一定要保证可以到达屏障点的并发任务数量是屏障宽度的整数倍，否则多出来的任务将一直等待。
 */
async fn barrier_vec() {
    let mut handles = Vec::with_capacity(10);

    // 参数10表示屏障宽度为10，只等待10个任务达到屏障点就放行这一批任务
    // 也就是说，某时刻已经有9个任务在等待，当第10个任务调用wait的时候，屏障将放行这一批
    let barrier = Arc::new(Barrier::new(10));

    for _ in 0..10 {
        let c = barrier.clone();
        handles.push(tokio::spawn(async move {
            println!("before wait");

            // 在此设置屏障，保证10个任务都已输出before wait才继续向下执行
            let wait_result = c.wait().await;
            println!("after wait");
            wait_result
        }));
    }

    let mut num_leaders = 0;
    for handle in handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

    assert_eq!(num_leaders, 1);
}

/**
 * 使用屏障时，一定要保证可以到达屏障点的并发任务数量是屏障宽度的整数倍，否则多出来的任务将一直等待。
 */
async fn barrier() {
    let barrier = Arc::new(Barrier::new(10));

    for i in 1..=15 {
        let b = barrier.clone();
        tokio::spawn(async move {
            // 任务的调度顺序是随机的
            println!("data before: {}", i);

            b.wait().await; // 15个任务中，多出5个任务将一直在此等待
            time::sleep(Duration::from_millis(10)).await;
            println!("data after: {}", i);
        });
    }
    time::sleep(Duration::from_secs(5)).await;
}
