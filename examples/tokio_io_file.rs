use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    read_file().await;
    read_lines().await;
    read_line().await;
}

/**
 * 本地文件IO
 */
async fn read_file() {
    let mut f = File::open("tmp/foo.txt").await.unwrap();
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await.unwrap();
    println!("The bytes: {:?}", &buffer[..n]);
}

/**
 * 本地文件IO：将File转换为BufReader将使得读取更为简便，可以直接 lines()按行读取文件
 */
async fn read_lines() {
    let file = tokio::fs::File::open("Cargo.toml").await.unwrap();
    // 将file转换为BufReader
    let mut buf_reader = tokio::io::BufReader::new(file).lines();
    // 每次读取一行
    while let Some(line) = buf_reader.next_line().await.unwrap() {
        // 注意lines()中的行是不带结尾换行符的，因此使用println!()而不是print!()
        println!("{}", line);
    }
}

/**
 * 本地文件IO：将File转换为BufReader将使得读取更为简便, 通过read_line()的方式来按行读取：
 */
async fn read_line() {
    let file = tokio::fs::File::open("Cargo.toml").await.unwrap();
    let mut buf_reader = tokio::io::BufReader::new(file);
    let mut buf = String::new();

    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(_e) => panic!("read file error"),
            // 遇到了文件结尾，即EOF
            Ok(0) => break,
            Ok(_n) => {
                // read_line()总是保留行尾换行符(如果有的话)，因此使用print!()而不是println!()
                print!("{}", buf);
                // read_line()总是将读取的内容追加到buf，因此每次读取完之后要清空buf
                buf.clear();
            }
        }
    }
}
