#![allow(dead_code)]
use std::{pin::Pin, io};

use futures::{pin_mut, Stream};

// Stream 并发：stream.try_for_each_concurrent()
async fn jump_around(stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
    stream.try_for_each_concurrent(100, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}

async fn jump_n_times(num: i32)-> Result<(), io::Error> {
    println!("jump_n_times :{}", num+1);
    Ok(())
}
async fn report_n_jumps(num: i32)-> Result<(), io::Error>{
    println!("report_n_jumps : {}", num*10);
    Ok(()) 
}


// async fn jump_around1(stream: impl Stream<Item=i32>) -> Result<(), io::Error> {
//     use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
//     // 不要忘记在迭代流之前固定（pin）它
//     pin_mut!(stream);
//     stream.try_for_each_concurrent(100, |num| async move {
//         jump_n_times(num).await?;
//         report_n_jumps(num).await?;
//         Ok(())
//     }).await?;

//     Ok(())
// }

async fn sum(stream: impl Stream<Item=usize>) -> usize {
    use futures::stream::StreamExt;
    pin_mut!(stream);
    let mut sum: usize = 0;
    while let Some(item) = stream.next().await {
        sum = sum + item;
    }
    sum
}

fn main() {

}