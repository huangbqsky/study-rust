use std::io::SeekFrom;

use tokio::{self, fs::File, io::{BufWriter, AsyncWriteExt, AsyncSeekExt, AsyncReadExt}, runtime};

fn main() {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(write());
    rt.block_on(write_all());
    rt.block_on(seek_file());
}

async fn write() {
    // 以write-only模式打开文件
    // 如果文件不存在，则创建，如果已存在，则截断文件
    let mut f = File::create("tmp/a.log").await.unwrap();

    let n = f.write(b"hello world!").await.unwrap();
    println!("write {} bytes", n);
}

async fn write_all() {
    let f = File::create("tmp/foo.txt").await.unwrap();
    let mut buffer = BufWriter::new(f);

    // 这次写入只是写入到缓冲空间
    buffer.write_all(b"some bytes").await.unwrap();

    println!("write_all {:?}", std::str::from_utf8(buffer.buffer()).unwrap());

    // 将缓冲空间的数据刷入writer
    buffer.flush().await.unwrap();

}

async fn seek_file() {
      // 只读方式打开文件时，偏移位置offset = 0
      let mut f = File::open("tmp/a.log").await.unwrap();
      let mut dst = String::new();
      let n = f.read_to_string(&mut dst).await.unwrap();
      println!("read_to_string {} bytes: {:?}", n, dst);

      // seek()设置offset = 4，从offset = 4开始读取，即从第5个字节开始读取
      // seek()返回设置后的偏移位置
      let n = f.seek(SeekFrom::Start(5)).await.unwrap();
      println!("seek SeekFrom::Start, offset = {}", n);
      
      let mut str = String::new();
      f.read_to_string(&mut str).await.unwrap();
      // 返回当前的偏移位置
      let n = f.stream_position().await.unwrap();
      println!("after read stream_position, offset = {}, data = {}", n, str);

      // 将偏移指针重置于offset = 0处
      f.rewind().await.unwrap();
      let n = f.stream_position().await.unwrap();
      println!("rewind, offset = {}", n);
}
