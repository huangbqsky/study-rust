#![allow(dead_code, deprecated, unused)]

use crossbeam;
use std::thread::{self, Thread};
use std::time::Duration;
use crossbeam_channel::{bounded, unbounded};

fn main() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    println!("max: {:?}", max);
    assert_eq!(max, Some(25));

    // test crossbeam channel
    // crossbeam_thread();

    // test crossbeam spsc channel 
    crossbeam_channel_spsc();
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2 ;
    let (left, right) = arr.split_at(mid);
    crossbeam::scope(|s| {
        let thread_left = s.spawn(|_| find_max(left));
        let thread_right = s.spawn(|_| find_max(right));

        let max_left = thread_left.join().unwrap()?;
        let max_right = thread_right.join().unwrap()?;
        Some(max_left.max(max_right))
    }).unwrap()
}

fn crossbeam_thread() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // 生产者线程
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
 
            // 关闭其中一个发送者 snd1
            // 该关闭操作对于结束最后的循环是必须的
            drop(snd1);
        });

        // 通过两个线程并行处理
        for _ in 0..n_workers {
            // 从数据源接收数据，然后发送到下沉端
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // 生成单独的工作线程
            s.spawn(move |_| {
            thread::sleep(Duration::from_millis(500));
                // 等待通道的关闭
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.",thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // 关闭通道，如果不关闭，下沉端将永远无法结束循环
        drop(snd2);

        // 下沉端
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    }).unwrap();
}

fn crossbeam_channel_spsc() {
    let (snd , rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
       s.spawn(|_| {
          for i in 0..n_msgs {
            snd.send(i).unwrap();
            println!("send msg {}", i);
            thread::sleep(Duration::from_millis(100));
          }
       });
    }).unwrap();

    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Recived {}", msg);
    }
}