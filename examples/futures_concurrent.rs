#![allow(dead_code)]
use std::{pin::Pin, io};

use futures::{pin_mut, Stream};
use futures::executor::block_on;

async fn jump_n_times(num: i32)-> Result<(), io::Error> {
    println!("jump_n_times :{}", num+1);
    Ok(())
}

async fn report_n_jumps(num: i32)-> Result<(), io::Error>{
    println!("report_n_jumps : {}", num*10);
    Ok(()) 
}


// Stream 并发: 参数要求是被 Pin 包裹的 trait 对象约束的 Stream，
async fn jump_around_pin_dyn(stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
    stream.try_for_each_concurrent(100, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}
// Stream 并发: 参数是 trait bounds 约束 的Stream， 所以需要 pin_mut宏 pin 住
async fn jump_around_trait_bound(stream: impl Stream<Item= Result<i32, io::Error>>) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
    // 不要忘记在迭代流之前固定（pin）它
    pin_mut!(stream);
    stream.try_for_each_concurrent(100, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}

async fn sum(stream: impl Stream<Item=usize>) -> usize {
    use futures::stream::StreamExt;
    pin_mut!(stream);
    let mut sum: usize = 0;
    while let Some(item) = stream.next().await {
        sum = sum + item;
    }
    sum
}

// 有一个有用的调试或一个简单的日志记录 inspect 组合子。它允许你传递一个 lambda，该 lambda 将通过引用接收流发出的每一项，而不会消耗该项。
async fn inspect(){
    use futures::*;
    let stream = stream::iter(vec![1, 2, 3]);
    let mut stream = stream.inspect(|val| println!("{}", val));
    assert_eq!(stream.next().await, Some(1));
    assert_eq!(stream.next().await, Some(2));
    assert_eq!(stream.next().await, Some(3));
    assert_eq!(stream.next().await, None);
}

fn main() {
    block_on(inspect());
}