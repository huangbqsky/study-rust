///! tokio::io::duplex()提供了类似套接字的全双工读写管道：
/// DuplexStream可读也可写，当管道为空时，读操作会进入等待，当管道空间已满时，写操作会进入等待。
/// DuplexStream实现了Send和Sync，因此可以跨线程、跨任务进行通信。
use chrono::Local;
use tokio::{self, runtime, time};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, DuplexStream};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

async fn write_duplex(r: &mut DuplexStream) -> io::Result<usize> {
    r.write(now().as_bytes()).await
}

async fn read_duplex(r: DuplexStream) {
    // split函数将DuplexStream分离为Reader和Writer，
    let (mut reader, writer) = tokio::io::split(r);
    // 不使用Writer，因此关闭Writer
    drop(writer);

    let mut buf = [0u8; 1024];
    loop {
        match reader.read(&mut buf).await {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if let Ok(data) = std::str::from_utf8(&buf[..n]) {
                    println!("read from duplex: {}", data);
                }
            }
        };
    }
}

// 模拟一个客户端和服务端，服务端向客户端循环不断地写入当前时间点，客户端不断读取来自服务端的数据并输出。
async fn read_write_duplex(){
    let (client, mut server) = tokio::io::duplex(64);

    // client read data from server
    tokio::spawn(async move {
        read_duplex(client).await;
    });

    // server write now() to client 
    loop {
        match write_duplex(&mut server).await {
            Err(_) | Ok(0) => break,
            _ => (),
        }
        time::sleep(time::Duration::from_secs(1)).await;
    }
}

// tokio::io::copy()方法可将Reader的所有数据(直到遇到EOF)直接拷贝给Writer。
async fn read_write_copy() {
    let mut reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = vec![];

    io::copy(&mut reader, &mut writer).await.unwrap();

    assert_eq!(&b"hello"[..], &writer[..]);
}


fn main() {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(read_write_copy());
    rt.block_on(read_write_duplex());
}