#![allow(dead_code)]
use std::cmp::Ordering;

type Link<T, U> = Option<Box<BST<T, U>>>;
// 二叉查找树

struct BST<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}
impl<T, U> BST<T, U>
where T: Clone + Ord + std::fmt::Debug,
      U: Clone + std::fmt::Debug {
    fn new() -> Self {
        BST { key: None, val: None, left: None, right: None }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none()
    }
    fn len(&self) -> usize {
        self.calc_len(0)
    }
    fn calc_len(&self, mut i: usize) -> usize {
        if self.key.is_none() {
            return i;
        }
        i += 1;
        if !self.left.is_none() {
            i = self.left.as_ref().unwrap().calc_len(i);
        }
        if !self.right.is_none() {
            i = self.right.as_ref().unwrap().calc_len(i);
        }
        i
    }
    fn preorder(&self) {
        println!("key: {:#?}, value: {:#?}", self.key, self.val);
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        match &self.right {
            Some(node) => node.preorder(),
            None => (),
        }

    }
    fn inorder(&self) {
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        println!("key: {:#?}, value: {:#?}", self.key, self.val);
        match &self.right {
            Some(node) => node.preorder(),
            None => (),
        }
    }
    fn postorder(&self) {
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        match &self.right {
            Some(node) => node.preorder(),
            None => (),
        }
        println!("key: {:#?}, value: {:#?}", self.key, self.val);
    }
    fn insert(&mut self, key: T, val: U) {
        if self.key.is_none() {
            self.key = Some(key);
            self.val = Some(val);
        } else {
            match &self.key {
                Some(k) => {
                    if key == *k {
                        self.val = Some(val);
                        return;
                    }

                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match child {
                        Some(ref mut node) => {
                            node.insert(key, val);
                        },
                        None => {
                            let mut node = BST::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
   
                        },
                    }
                },
                None => (),
            }
        }
    }

    fn search(&self, key: &T) -> bool {
        match &self.key {
            Some(k) => {
                match k.cmp(&key) {
                    Ordering::Equal => {true},
                    Ordering::Greater => {
                      match &self.left {
                        Some(node) => node.search(key),
                        None => false,
                      }
                    },
                    Ordering::Less => {
                        match &self.right {
                            Some(node) => node.search(key),
                            None => false,
                        }

                    }
                }
            },
            None => false,
        }
    }

    fn min(&self) -> (Option<&T>, Option<&U>){
        match &self.left {
            Some(node) => node.min(),
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    fn max(&self) -> (Option<&T>, Option<&U>) {
        match &self.right {
            Some(node) => node.max(),
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            }
        }
    }
    fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            None => None,
            Some(k) => {
                match k.cmp(&key) {
                    Ordering::Equal => self.val.as_ref(),
                    Ordering::Greater => {
                        match &self.left {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    },
                    Ordering::Less => {
                        match &self.right {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    }
                }
            }
        }
    }

}

fn main() {
    let mut bst = BST::<i32, char>::new();
    bst.insert(8,'e'); bst.insert(6,'c'); bst.insert(7,'d');
    bst.insert(5,'b'); bst.insert(10,'g'); bst.insert(9,'f');
    bst.insert(11,'h'); bst.insert(4,'a');

    println!("empty: {:?}, len: {:?}", bst.is_empty(), bst.len());
    println!("max: {:?}, min: {:?}", bst.max(), bst.min());
    println!("key: 5, value: {:?}", bst.get(&5));
    println!("5 in bst: {:?}", bst.search(&5));

    println!("inorder: {:?}", bst.inorder());
    println!("preorder: {:?}", bst.postorder());
    println!("postorder: {:?}", bst.postorder());

}