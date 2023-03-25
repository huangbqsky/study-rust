use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

/// 单链表节点
#[derive(Debug)]
struct Node<T>{
    data: T,
    next: Link<T>,
}

/// 单链表
#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
}
impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}
impl<T> LinkedList<T> {
    fn new() -> Self {
      Self{head: None}
    }

    fn prepend(&mut self, data: T) -> &mut Self {
        // 从传入数据构建要插入的节点
        let mut new_node = Box::new(Node::new(data));
        match self.head {
            // 当前链表为空时，插入的节点直接作为头节点
            None => self.head = Some(new_node),
            Some(_) => {
                // 调用Option的take方法取出Option中的头节点，作为新插入节点的下一个节点
                new_node.next = self.head.take();
                // 讲新插入的节点作为链表的头节点
                self.head = Some(new_node);
            }
        }
        self
    }
    fn reverse(&mut self) {
       let mut prev = None;
       while let Some(mut node) = self.head.take(){
           self.head = node.next;
           node.next = prev;
           prev = Some(node);
       }
       self.head = prev;
    }
}
impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       if self.head.is_none() {
           // 如果链表为空，只打印None
           write!(fmt, "None\n")?;
       } else {
           // 下面将遍历链表，因为只是打印，能获取链表哥哥节点数据就行，所以不需要获取所有权
           let mut next = self.head.as_ref();
           while let Some(node) = next {
              write!(fmt, "{} -> ", node.data)?;
              next = node.next.as_ref();
           }
           write!(fmt, "None\n")?;
       }
       Ok(())
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.prepend(5).prepend(4).prepend(3).prepend(2).prepend(1);
    println!("{ll}");
    ll.reverse();
    println!("{ll}");
}
