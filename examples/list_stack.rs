
#![allow(dead_code)]

type Link<T> = Option<Box<Node<T>>>;


// 链表节点
#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        // 初始化是无下一个链接
        Node {data: val, next: None }
    }
}

// 链表栈
#[derive(Debug, Clone)]
struct Stack<T> {
    size: usize,
    top: Link<T>, // 栈顶控制整个栈
}
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { size: 0, top: None }
    }
    // take取出 top中节点，留下空位，所以可以回填节点
    fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }
    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| {
            &node.data
        })
    }
    fn size(&self) -> usize {
        self.size
    }
    fn is_empty(&self) -> bool {
        0 == self.size
    }
}
fn main() { 
    let mut s = Stack::new();
    s.push(1); s.push(2); s.push(4); 
    println!("top: {:?}, size {}", s.peek().unwrap(), s.size);
    println!("top: {:?}, size {}", s.pop().unwrap(), s.size);
    println!("is_empty: {:?}, stack:{:?}", s.is_empty(), s);

}