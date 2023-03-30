#![allow(dead_code)]
use futures::{join, try_join, future::TryFutureExt};

struct Book;
struct Music;

//--------------------------join------------------------------
async fn enjoy_book() -> Book { /* ... */ Book }
async fn enjoy_music() -> Music { /* ... */ Music}

// 内部是顺次执行，而不是同时运行它们
async fn enjoy1_book_and_music() -> (Book, Music) {
    // 实际在异步函数内部是是串型执行
    let book = enjoy_book().await; // await触发阻塞式执行
    let music = enjoy_music().await; // await触发阻塞式执行
    (book, music)
}
// 内部是顺次执行，而不是同时运行它们
async fn enjoy2_book_and_music() -> (Book, Music) {
    // 实际在异步函数内部是是串型执行
    let book_future = enjoy_book(); // 异步函数是惰性的，没有执行
    let music_future = enjoy_music(); // 异步函数是惰性的，没有执行
    (book_future.await, music_future.await)
}
// 正确的并发运行两个 Future ，来试试 futures::join! 宏
async fn enjoy_book_and_music() -> (Book, Music) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    // join!必须等待它管理的所有 Future 完成后才能完成
    join!(book_fut, music_fut)
}

//--------------------------try_join------------------------------
async fn get_book() -> Result<Book, ()> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

// 某一个 Future 报错后就立即停止所有 Future 的执行，可以使用 try_join!
// async fn get_book_and_music() -> Result<(Book, Music), String> {
//     let book_fut = get_book();
//     let music_fut = get_music();
//     try_join!(book_fut, music_fut)
// }

/**
 * 传给 try_join! 的所有 Future 都必须拥有相同的错误类型。
 * 如果错误类型不同，可以考虑使用来自 futures::future::TryFutureExt 模块的 map_err和 err_info方法将错误进行转换:
 */
async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music();
    // 某一个 Future 报错后就立即停止所有 Future 的执行，可以使用 try_join!
    try_join!(book_fut, music_fut)
}

async fn get_into_book_and_music() -> (Book, Music) {
    get_book_and_music().await.unwrap()
}

fn main() {
    futures::executor::block_on(enjoy_book_and_music());
    futures::executor::block_on(get_into_book_and_music());
}