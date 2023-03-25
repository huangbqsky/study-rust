#![allow(dead_code, unused_imports)]
use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;
use num::complex::Complex;
use image::{ImageBuffer, Pixel, Rgb};
use anyhow::Result;

fn main() -> Result<()> {
    // let (width, height) = (1920, 1080);
    // // 为指定宽高的输出图片分配内存
    // let mut img = ImageBuffer::new(width, height);
    // let iterations = 300;

    // let c = Complex::new(-0.8, 0.156);

    // let pool = ThreadPool::new(num_cpus::get());
    // let (tx, rx) = channel();

    // for y in 0..height {
    //     let tx = tx.clone();
    //     // execute 将每个像素作为单独的作业接收
    //     pool.execute(move || for x in 0..width {
    //                      let i = julia(c, x, y, width, height, iterations);
    //                      let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
    //                      tx.send((x, y, pixel)).expect("Could not send data!");
    //                  });
    // }

    // for _ in 0..(width * height) {
    //     let (x, y, pixel) = rx.recv()?;
    //     // 使用数据来设置像素的颜色
    //     img.put_pixel(x, y, pixel);
    // }
    
    // // 输出图片内容到指定文件中
    // let _ = img.save("output.png")?;
    Ok(())
}