#![allow(dead_code, unused)]
use std::pin::Pin;

use futures::{future, Future};
use futures::{executor, pin_mut, select};
use futures::future::{Fuse, FusedFuture, FutureExt};
use futures::stream::{FusedStream, FuturesUnordered, Stream, StreamExt};


async fn task_one() { /* ... */ }
async fn task_two() { /* ... */ }

/**
 * 赛跑模式：同时并发地运行 t1 和 t2， 无论两者哪个先完成, 函数结束且不会等待另一个任务的完成
 */
async fn race_tasks() {
    // .fuse() 方法可以让 Future 实现 FusedFuture 特征，
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    // pin_mut 宏会为 Future 实现 Unpin特征
    pin_mut!(t1, t2);

    // 同时等待多个 Future ，且任何一个 Future 结束后，都可以立即被处理，可以考虑使用 futures::select!:
    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
        complete => println!("race_tasks completed!"),
        default => println!("race_tasks default!"),
    }
}

/**
 * select 宏所必须的满足: FusedStream + Unpin， 通过 fuse 方法和 pin_mut 宏实现
 * 
1. Unpin，由于 select 不会通过拿走所有权的方式使用Future，而是通过可变引用的方式去使用，这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以继续被其它代码使用。
2. FusedFuture的原因跟上面类似，当 Future 一旦完成后，那 select 就不能再对其进行轮询使用。Fuse意味着熔断，相当于 Future 一旦完成，再次调用poll会直接返回Poll::Pending。
 */
async fn add_two_streams() -> u8 {
    // mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    // mut s2: impl Stream<Item = u8> + FusedStream + Unpin,

    // .fuse() 方法可以让 Stream 实现 FusedStream 特征，
    let s1 = futures::stream::once(async { 10 }).fuse();
    let s2 = futures::stream::once(async { 20 }).fuse();

    // pin_mut 宏会为 Stream 实现 Unpin 特征
    pin_mut!(s1, s2);

    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
            // default => panic!(), // 该分支永远不会运行，因为`Future`会先运行，然后是`complete`
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }
    println!("add_two_streams，total = {total}");
    total
}

/**
 * 
select!还支持 default 和 complete 分支:
1. complete 分支当所有的 Future 和 Stream 完成后才会被执行，它往往配合 loop 使用，loop 用于循环完成所有的 Future
2. default 分支，若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行
 */
async fn futures_select (){
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            default => println!("race_tasks default!"),
            complete => { println!("futures_select completed!"); break },
            // default => panic!(), // 该分支永远不会运行，因为`Future`会先运行，然后是`complete`
        };
    }
    assert_eq!(total, 10);
}

async fn future_in_select() {
    // 创建一个空 Future
    let fut = Fuse::terminated();
    // 创建一个 FuturesUnordered 类型，可以多次拷贝
    let mut async_tasks: FuturesUnordered<Pin<Box<dyn Future<Output = i32>>>> = FuturesUnordered::new();
    async_tasks.push(Box::pin(async { 1 }));
    
    pin_mut!(fut);
    
    let mut total = 0;
    loop {
        select! {
            // select_next_some 函数可以用在 `select` 上，并且只运行从 stream 返回的 `Some(_)` 值而忽略 `None`
            num = async_tasks.select_next_some() => {
                println!("first num is {num} and total is {total}");
                total += num;
                println!("total is {total}");
                if total >= 10 { break; }
                // 判断是否已经终止
                if fut.is_terminated() {
                    // 设置一个按需填充新 future
                    fut.set(async { 1 }.fuse());
                }
            },
            num = fut => {
                println!("second num is {num} and total is {total}");
                total += num;
                println!("now total is {total}");
                async_tasks.push(Box::pin(async { 1 }));
            },
            complete => break,
            default => panic!(),
        };
    }

    println!("total finally is {total}");
}

fn main() {
    executor::block_on(race_tasks());
    executor::block_on(add_two_streams());
    executor::block_on(future_in_select());
    
    futures_select ();
}

  