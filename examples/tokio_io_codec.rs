use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};

use tokio::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{self, Framed};

use serde::{Deserialize, Serialize};
use bytes::{BufMut, BytesMut};
use log::{debug, error};

// 类型别名
type RstRespFramedStream = SplitStream<Framed<TcpStream, RstRespCodec>>;
type RstRespFramedSink = SplitSink<Framed<TcpStream, RstRespCodec>, RstResp>;

/// 请求
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub sym: String,
    pub from: u64,
    pub to: u64,
}

/// 响应
#[derive(Debug, Serialize, Deserialize)]
pub struct Response();
// pub struct Response(pub Option<Klines>);

/// 对请求和响应的封装，之后客户端和服务端都将通过Sink和Stream来基于该类型通信
#[derive(Debug, Serialize, Deserialize)]
pub enum RstResp {
    Request(Request),
    Response(Response),
}

/// 自己定义一个Codec，并实现codec的Decoder和Encoder，完成 RstResp => &[u8] => RstResp 之间的转换
/// 1. Decoder：将二进制字节数据帧转换为指定的Rust类型
/// 2. Encoder：将指定的Rust类型转换为二进制字节数据帧
/// 
/// 一个简单、通用且常用的二进制通信协议格式是：| data_len | data |，即：
/// 1. 在编码时(Encoder)，先计算待发送的二进制数据data的长度，并将长度大小放在帧首，实际数据放在长度的后面，这是一个完整的帧
/// 2. 在解码时(Decoder)，先读取长度大小，根据读取的长度大小再先后读取指定数量的字节，从而读取一个完整的帧，再将其转换为指定的数据类型
pub struct RstRespCodec;
impl RstRespCodec {
    /// 最多传送1G数据
    const MAX_SIZE: usize = 1024 * 1024 * 1024 * 8;
}

/// 实现Encoder，将RstResp转换为字节数据
/// 对于codec而言，直接将二进制数据写入 `dst: &mut BytesMut` 即可
impl codec::Encoder<RstResp> for RstRespCodec {
    type Error = bincode::Error;
    // 本示例中使用bincode将RstResp转换为&[u8]，也可以使用serde_json::to_vec()，前者效率更高一些
    fn encode(&mut self, item: RstResp, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let data = bincode::serialize(&item)?;
        let data = data.as_slice();

        // 要传输的实际数据的长度
        let data_len = data.len();
        if data_len > Self::MAX_SIZE {
            return Err(bincode::Error::new(bincode::ErrorKind::Custom(
                "frame is too large".to_string(),
            )));
        }

        // 最大传输u32的数据(可最多512G)，
        // 表示数据长度的u32数值占用4个字节
        dst.reserve(data_len + 4);

        // 先将长度值写入dst，即帧首，
        // 写入的字节序是大端的u32，读取时也要大端格式读取，
        // 也有小端的方法`put_u32_le()`，读取时也得小端读取
        dst.put_u32(data_len as u32);

        // 再将实际数据放入帧尾
        dst.extend_from_slice(data);
        println!("encode RstResp {:?}", dst);
        Ok(())
    }
}

/// 实现Decoder，将字节数据转换为RstResp
impl codec::Decoder for RstRespCodec {
    type Item = RstResp;
    type Error = std::io::Error;
    // 从不断被填充的Bytes buf中读取数据，并将其转换到目标类型
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let buf_len = src.len();

        // 如果buf中的数据量连长度声明的大小都不足，则先跳过等待后面更多数据的到来
        if buf_len < 4 {
            return Ok(None);
        }

        // 先读取帧首，获得声明的帧中实际数据大小
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&src[..4]);
        let data_len = u32::from_be_bytes(length_bytes) as usize;
        if data_len > Self::MAX_SIZE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", data_len),
            ));
        }

        // 帧的总长度为 4 + frame_len
        let frame_len = data_len + 4;

        // buf中数据量不够，跳过，并预先申请足够的空闲空间来存放该帧后续到来的数据
        if buf_len < frame_len {
            src.reserve(frame_len - buf_len);
            return Ok(None);
        }

        // 数据量足够了，从buf中取出数据转编成帧，并转换为指定类型后返回
        // 需同时将buf截断(split_to会截断)
        let frame_bytes = src.split_to(frame_len);
        match bincode::deserialize::<RstResp>(&frame_bytes[4..]) {
            Ok(frame) => Ok(Some(frame)),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    while let Ok((client_stream, client_addr)) = server.accept().await {
        println!("accept client: {}", client_addr);
        tokio::spawn(async move {
            process_client(client_stream).await;
        });
    }
}

async fn process_client(client_stream: TcpStream) {
    // 将TcpStream转换为Framed
    let framed = Framed::new(client_stream, RstRespCodec);
    // 将Framed分离，可得到独立的读写端
    let (frame_writer, frame_reader) = framed.split::<RstResp>();
    // 当Reader从客户端读取到数据后，发送到通道中，
    // 另一个异步任务读取该通道，从通道中读取到数据后，将内容按行写给客户端
    let (msg_tx, msg_rx) = mpsc::channel::<RstResp>(100);

    // 负责读客户端的异步子任务
    let mut read_task = tokio::spawn(async move {
        read_from_client(frame_reader, msg_tx).await;
    });

    // 负责向客户端写行数据的异步子任务
    let mut write_task = tokio::spawn(async move {
        write_to_client(frame_writer, msg_rx).await;
    });

    // 无论是读任务还是写任务的终止，另一个任务都将没有继续存在的意义，因此都将另一个任务也终止
    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        eprintln!("read_task/write_task terminated");
        read_task.abort();
        write_task.abort();
    };
}

// 负责读的
async fn read_from_client(mut frame_reader: RstRespFramedStream, msg_tx: mpsc::Sender<RstResp>) {
    loop {
        match frame_reader.next().await {
            None => {
                debug!("peer closed");
                println!("client closed");
                break;
            }
            Some(Err(e)) => {
                error!("read peer error: {}", e);
                eprintln!("read from client error: {}", e);
                break;
            }
            Some(Ok(req_resp)) => {
                println!("read from client. content: {:?}", req_resp);
                match req_resp {
                    RstResp::Request(_) => println!("read from Request. content: {:?}", req_resp),
                    RstResp::Response(_) => println!("read from Response. content: {:?}", req_resp),
                };

                // 将内容发送给writer，让writer响应给客户端，
                // 如果无法发送给writer，继续从客户端读取内容将没有意义，因此break退出
                if msg_tx.send(req_resp).await.is_err() {
                    eprintln!("receiver closed");
                }
            }
        }
    }
}

// 负责写的
async fn write_to_client(mut frame_writer: RstRespFramedSink, mut msg_rx: mpsc::Receiver<RstResp>) {
 
    // let resp = RstResp::Response(resp);
    // if frame_writer.send(resp).await.is_err() {
    //     error!("write failed");
    // }

    while let Some(resp) = msg_rx.recv().await {
        if frame_writer.send(resp).await.is_err() {
            eprintln!("write to client failed");
            break;
        }
    }
}
