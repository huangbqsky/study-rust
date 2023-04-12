#![allow(dead_code)]
use std::ops::Deref;

#[derive(Debug)]
// 自引用结构体
pub struct SelfRef<'a> {
    str: String,
    str_ref: Option<&'a str>,
}

impl SelfRef<'_> {
    fn change_str(&mut self) { // 可变借用 self_ref

    }
}

fn move_and_print<'a>(self_ref: SelfRef<'a>) { // 不能移动 self_ref
    println!("{:?}", self_ref);
    println!("{:p}", self_ref.str.deref());
    println!("{:p}", self_ref.str_ref.unwrap());
}

fn main() {
    let str = "Hello".to_string();
    let mut self_ref = SelfRef { str, str_ref: None };
    self_ref.str_ref = Some(self_ref.str.deref()); // self_ref是可变借用，并且str_ref字段指向的值是就是str字段
    println!("{:?}", self_ref);
    println!("{:p}", self_ref.str.deref()); // str解引用 ，比如：0x12be06c90
    println!("{:p}", self_ref.str_ref.unwrap()); // str_ref指针， 比如：0x12be06c90
    
    // 1.无法获取可变引用 ： 不能借用`self_ref`作为可变，因为它已经被借用为不可变
    // self_ref.change_str(); // error：cannot borrow `self_ref` as mutable because it is also borrowed as immutable

    // 2.无法移动：不能移动 self_ref ，因为它已经被借用
    // move_and_print(self_ref); // error：cannot move out of `self_ref` because it is borrowed


    // 裸指针实现的自引用
    let mut self_ref = SelfRefData::new(Data { data: 1 });
    self_ref.init();
    self_ref.print_info();
}

pub struct Data {
    data: usize,
}
// 自引用结构体（裸指针实现）
pub struct SelfRefData {
    data: Data,
    data_ref: *const Data, // 裸指针实现
}

impl SelfRefData {
    fn new(data: Data) -> Self {
        Self {
            data,
            data_ref: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        self.data_ref = &self.data;
    }

    fn print_info(&self) {
        println!(
            "data address: {:p} content: {}，data_ref：{:p} content: {}",
            &self.data,
            self.data.data,
            self.data_ref,
            unsafe { &*self.data_ref }.data
        );
    }
}