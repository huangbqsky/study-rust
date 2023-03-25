#![allow(dead_code)]

#[derive(Debug)]
struct Deque<T> {
   cap: usize, // 容量
   data: Vec<T>, // 容量数据
}
impl<T> Deque<T> {
    fn new(capacity: usize) -> Self {
       Deque {
        cap: capacity,
        data: Vec::with_capacity(capacity),
       }
    }
    // Vec 末尾为队首
    fn add_front(&mut self, val: T) -> Result<(), String>{
        if Self::size(&self) == self.cap {
           return Err("No Space available".to_string());
        }
        self.data.push(val);
        Ok(())
    }
    // Vec 首部为队尾
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No Space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    fn remove_front(&mut self) -> Option<T> {
       if Self::size(&self) > 0 {
          self.data.pop()
       } else {
          None
       }
    }
    // 从对尾移除数据
    fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
       0 == Self::size(&self)
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1); 
    let _r2 = d.add_front(2);
    let _r3 = d.add_rear(3);
    let _r3 = d.add_rear(4);

    println!("deque: {:?}", d);

    if let Err(error) = d.add_front(5) {
        println!("add front error: {error}")
    }

    if let Some(data) = d.remove_rear() {
        println!("data: {:?}", data);
    } else {
        println!("empty deque");
    }

    println!("size: {}, is_empty: {}", d.size(), d.is_empty());
    println!("contents: {:?}", d);
}