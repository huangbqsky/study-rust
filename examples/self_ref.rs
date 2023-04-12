#![allow(dead_code)]
use std::{ops::{Deref, DerefMut}, mem};

#[derive(Debug)]
// 自引用结构体
pub struct SelfRefStr<'a> {
    str: String,
    str_ref: Option<&'a str>,
}

impl SelfRefStr<'_> {
    fn change_str(&mut self) { // 可变借用 self_ref

    }
}

fn move_and_print<'a>(self_ref: SelfRefStr<'a>) { // 不能移动 self_ref
    println!("{:?}", self_ref);
    println!("{:p}", self_ref.str.deref());
    println!("{:p}", self_ref.str_ref.unwrap());
}

fn main() {
    println!("{}", "-".repeat(100));
    let str = "Hello".to_string();
    let mut self_ref = SelfRefStr { str, str_ref: None };
    self_ref.str_ref = Some(self_ref.str.deref()); // self_ref是可变借用，并且str_ref字段指向的值是就是str字段
    // println!("{:?}", self_ref);
    // println!("{:p}", self_ref.str.deref()); // str解引用 ，比如：0x12be06c90
    // println!("{:p}", self_ref.str_ref.unwrap()); // str_ref指针， 比如：0x12be06c90
    println!(
        "{:?}, str address: {:p}, str_ref address: {:p}",
        self_ref,
        self_ref.str.deref(),
        self_ref.str_ref.unwrap(),
    );
    
    // 1.无法获取可变引用 ： 不能借用`self_ref`作为可变，因为它已经被借用为不可变
    // self_ref.change_str(); // error：cannot borrow `self_ref` as mutable because it is also borrowed as immutable

    // 2.无法移动：不能移动 self_ref ，因为它已经被借用
    // move_and_print(self_ref); // error：cannot move out of `self_ref` because it is borrowed


    // 裸指针实现的自引用
    // let mut self_ref = SelfRefData::new(Data { data: 1 });
    // self_ref.init();
    // self_ref.print_info();

    // 在栈上内存移动非常普遍，test方法返回了SelfRef，此时就移动了SelfRef
    println!("{}", "-".repeat(100));
    let self_ref = test();
    self_ref.print_info();

    // 在堆上移动自引用结构
    println!("{}", "-".repeat(100));
    let self_ref = test_box();
    self_ref.print_info();

    // mem::swap交换了2个在堆上的自引用结构的内存
    println!("{}", "-".repeat(100));
    mem_swap ();
}

#[derive(Clone, Copy, Debug)]
pub struct Data {
    data: usize,
}
// 自引用结构体（裸指针实现）
#[derive(Clone, Copy, Debug)]
pub struct SelfRef {
    data: Data,
    data_ref: *const Data, // 裸指针实现
}

// 在栈上内存移动非常普遍，下面定义了test方法，返回了SelfRef，此时就移动了SelfRef
fn test() -> SelfRef {
    let mut self_ref = SelfRef::new(Data { data: 1 });
    self_ref.init();
    self_ref.print_info();
    self_ref
}

// 自引用结构在堆上分配了，移动Box<SelfRef>只会移动栈上的数据，堆上的数据并没有发生移动
fn test_box() -> Box<SelfRef> {
    let mut self_ref = Box::new(SelfRef::new(Data { data: 1 }));
    self_ref.init();
    self_ref.print_info();
    self_ref
}
// mem::swap交换了2个在堆上的自引用结构的内存，自然发生了移动
// 但是这个时候出现的问题不是悬空指针，而是数据出现了错乱，指向的不在是自己结构的data字段
fn mem_swap (){
    let mut self_ref = Box::new(SelfRef::new(Data { data: 1 }));
    self_ref.init();
    let mut self_ref2 = Box::new(SelfRef::new(Data { data: 2 }));
    self_ref2.init();

    self_ref.print_info(); // 交换前信息
    self_ref2.print_info(); // 交换前信息

    // swap交换内存
    mem::swap(self_ref.deref_mut(),self_ref2.deref_mut());

    self_ref.print_info(); // 交换后信息
    self_ref2.print_info(); // 交换后信息
}

impl SelfRef {
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
            "{:?}, data address: {:p} content: {}，data_ref：{:p} content: {}",
            &self,
            &self.data,
            self.data.data,
            self.data_ref,
            unsafe { &*self.data_ref }.data
        );
    }
}