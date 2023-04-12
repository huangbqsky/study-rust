
#[derive(Debug)]
// 自引用结构体
pub struct SelfRef<'a> {
    str: String,
    str_ref: &'a str,
}
fn main() {
    let str = "Hello".to_string();
    let self_ref = SelfRef { str, str_ref: &str };
    println!("{:?}", self_ref);
}