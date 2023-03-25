#![allow(dead_code)]

// 节点连接用Box指针（大小确定），因为大小确定的才能分派内存
type Link<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    size: usize,  // 链表节点数
    head: Link<T>, // 头节点
}
struct Node<T> {
    elem: T,  // 数据
    next: Link<T>, // 下一个节点连接
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { size: 0, head: None }
    }
    pub fn is_empty(&self) -> bool {
        0 == self.size
    }
    pub fn size(&self) -> usize {
        self.size
    }
    // 新节点总是添加到头部
    pub fn push(&mut self, val: T) {
        let node = Box::new(Node {
            elem: val,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }
    // take 会取出数据并留留下空位
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem

        })
    }
    // peek 不改变值，只返回引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    // peek_mut可改变值，是可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
    // into_iter链表改变，成为迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    // iter 链表不变，只得到不可变迭代器
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
    // iter_mut 链表不变，得到可变迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
pub struct Iter<'a, T: 'a> { next: Option<&'a Node<T>>} 
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}
pub struct IterMut<'a, T: 'a> { next: Option<&'a mut Node<T>>}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
fn main() { 
    fn basics() {
      let mut list = List::new();
      list.push(1);
      list.push(2);
      list.push(3);
      assert_eq!(list.pop(), Some(3));
      assert_eq!(list.peek(), Some(&2));
      assert_eq!(list.peek_mut(), Some(&mut 2));
      list.peek_mut().map(|val| {
        *val = 4;
      });
      assert_eq!(list.peek(), Some(&4));
      println!("basics test ok");
    }

    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        println!("into_iter test ok!");
    }
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        println!("Iter text ok!");
    }

    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
        println!("iter_mut test Ok!");
    }

    basics();
    into_iter();
    iter();
    iter_mut();

}