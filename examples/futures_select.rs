#![allow(dead_code)]
use futures::{executor, pin_mut, select, FutureExt};


async fn task_one() { /* ... */ }
async fn task_two() { /* ... */ }

/**
 * 赛跑模式：同时并发地运行 t1 和 t2， 无论两者哪个先完成, 函数结束且不会等待另一个任务的完成
 */
async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    // 同时等待多个 Future ，且任何一个 Future 结束后，都可以立即被处理，可以考虑使用 futures::select!:
    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    }
}

fn main() {
    executor::block_on(race_tasks());
}