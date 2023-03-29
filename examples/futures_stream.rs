use std::pin::Pin;
use futures::Stream;
use futures::io;
use futures::channel::mpsc;
use futures::executor::block_on;

// Stream 的发送和接收
async fn send_recv() {
    use futures::StreamExt; // 引入 next
    use futures::SinkExt; // 引入 send
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
    println!("rx: {:?} -> {:?} -> {:?}", next_one, next_two, next_none);

    assert_eq!(Some(1), next_one);
    assert_eq!(Some(2), next_two);
    assert_eq!(None, next_none);
}

// 代码使用 stream::iter 生成了一个 Stream，并对其进行 filter / map 的操作。最后，遍历整个 stream，把获得的数据打印出来。
async fn stream_iter(){
    use futures::future; // 引入 ready
    use futures::stream; // 引入 iter
    use futures::stream::StreamExt; // 引入 filter

    // 使用 stream::iter() 函数创建一个流
    let mut st = stream::iter(1..10)
    .filter(|x| future::ready(x % 2 == 0))
    .map(|x| x * x);

    while let Some(x) = st.next().await {
       println!("Got item: {}", x);
    }
}

// Stream 迭代：stream.next()
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt; // 引入 next
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}
// Stream 迭代：stream.try_next()
async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // 引入 try_next
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}

// Stream 并发：stream.try_for_each_concurrent()
async fn jump_around (
    stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream
        .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
            jump_n_times(num).await?;
            report_n_jumps(num).await?;
            Ok(())
        })
        .await?;

    Ok(())
}

async fn jump_n_times(num: i32)-> Result<(), io::Error> {
    println!("jump_n_times :{}", num+1);
    Ok(())
}
async fn report_n_jumps(num: i32)-> Result<(), io::Error>{
    println!("report_n_jumps : {}", num);
    Ok(()) 
}

// 使用 repeat_with 创建 stream，无法控制何时结束
fn fib() -> impl Stream<Item = i32> {
    let mut a = 1;
    let mut b = 2;

    futures::stream::repeat_with(move || {
        let c = a + b;
        a = b;
        b = c;
        b
    })
}

async fn consume(mut st: impl Stream<Item = i32> + Unpin) {
    use futures::stream::StreamExt; // 引入 next
    while let Some(v) = st.next().await {
        print!("{} ", v);
    }
    print!("\n");
}


async fn stream_repeat_with(){
    use futures::stream::StreamExt; // 引入 next

    let mut curr = 1;
    let mut pow2 = futures::stream::repeat_with(|| { let tmp = curr; curr *= 2; tmp });

    assert_eq!(Some(1), pow2.next().await);
    assert_eq!(Some(2), pow2.next().await);
    assert_eq!(Some(4), pow2.next().await);
    assert_eq!(Some(8), pow2.next().await);
}

fn main() {
    block_on(send_recv());
    block_on(stream_iter());
    block_on(stream_repeat_with()); 
}
