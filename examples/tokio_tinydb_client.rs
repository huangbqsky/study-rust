#![warn(rust_2018_idioms)]
#![allow(dead_code, unused)]

use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use bytes::{Bytes, BytesMut, BufMut, Buf};
use futures_util::{SinkExt, StreamExt};
use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

   
    //                INPUT
    // +---- len -----+- head -+--- Payload ---+
    // | \x00\x00\x0B | \x0C |  Hello world  |
    // +--------------+--------+---------------+
    //                 DECODED
    // +---- len -----+- head -+--- Payload ---+
    // | \x00\x00\x0B | \x0C |  Hello world  |
    // +--------------+--------+---------------+
     
    let codec = LengthDelimitedCodec::builder()
        .length_field_offset(0)
        .length_field_length(3)
        .length_adjustment(1)
        .num_skip(0)
        .new_codec();

    let mut stream = Framed::new(stream, codec);

    println!("created stream");

    let mut cmd = BytesMut::new();
    // cmd.reserve(1);
    cmd.put_u8(9);
    let msg = "SET foo far".as_bytes();
    // cmd.reserve(1 + msg.len());
    cmd.put_slice(msg);

    println!("cmd len= {}, cmd capacity= {}, cmd: {:?}", cmd.len(), cmd.capacity(), cmd);
     // let result = stream.send(cmd.to_bytes()).await;
    let frame = Bytes::from(cmd);
    println!("send frame = {:?}", frame);
    let result = stream.send(frame).await;
    println!("wrote to stream; success={}", result.is_ok());

    // Read the first line from the `LineCodec` stream to get the username.
    let result = match stream.next().await {
        Some(Ok(line)) => {
            line
        },
        // We didn't get a line so we return early here.
        _ => {
            println!("Failed to get username from. Client disconnected.");
            return Ok(());
        }
    };
    println!("result line: {:?}", result);
    let mut line = BytesMut::new();
    line.put_slice(&result);
    let len = line.get_uint(3);
    let seq = line.get_uint(1);
    let payload = line.chunk();
    // println!("{},{},{}", len, seq, String::from_utf8_lossy(payload.bytes()));
    println!("len: {}, seq: {}, payload: {}", len, seq, String::from_utf8_lossy(payload));

    Ok(())
}