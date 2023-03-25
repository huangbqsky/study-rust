#![allow(dead_code)]

#[derive(Debug)]
pub struct Stack<T> {
  top: usize, // 栈顶
  data: Vec<T>, // 栈数据容器
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { 
            top: 0, 
            data: Vec::new() 
        }
    }
    pub fn push(&mut self, val: T) { 
        self.data.push(val); // 保存数据再Vec末尾
        self.top += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1; // 栈顶减1后再弹出数据
        self.data.pop()
    }
    pub fn peek(& self) -> Option<&T> { // 数据不能移动，只能返回引用
        if self.top == 0 { return None; }
        self.data.get(self.top -1)
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    pub fn size(&self) -> usize {
        self.top  // 栈 顶 恰 好 就 是 栈 中 元 素 个 数
    }
}

fn par_match(open: char, close: char) -> bool {
   let opens = "([{";
   let closers = ")]}";
   opens.find(open) == closers.find(close)
}
fn par_checker3(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }
    println!("current char_list: {:?}", char_list);

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len()  && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
        println!("current index :{index}, stack: {:?}", stack);
    }
    
    balance && stack.is_empty()
}

fn main() {
    let mut s = Stack::new();
    s.push(1); s.push(2); s.push(4);
    println!("top {:?}, size {}", s.peek().unwrap(), s.size());
    println!("pop {:?}, size {}", s.pop().unwrap(), s.size());
    println!("is_empty:{}, stack:{:?}", s.is_empty(), s);


    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = par_checker3(sa);
    let res2 = par_checker3(sb);
    println!("sa balanced: {res1}, sb balanced: {res2}");
}