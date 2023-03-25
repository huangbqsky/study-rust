#![allow(dead_code)]

#[derive(Debug)]
struct Queue<T> {
   cap: usize, //容量
   data: Vec<T>, // 数据容器
}
impl<T> Queue<T> { 
    fn new(size: usize) -> Self {
        Queue { cap: size, data: Vec::with_capacity(size) }
    }
    // 判断是否有剩余空间，有则数据加入收据
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No Space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    // 数据出队列
    fn dequeue(&mut self) -> Option<T> {
       if Self::size(&self) > 0 {
          self.data.pop()
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
   let mut q = Queue::new(3);
   let _r1 = q.enqueue(1);
   let _r2 = q.enqueue(2);
   let _r3 = q.enqueue(3);

   if let Err(e) = q.enqueue(4) {
       println!("Enqueue error: {e}");
   }

   if let Some(data) = q.dequeue() {
      println!("data:{data}");
   } else {
      println!("empty queue");
   }

   println!("size: {}, empty: {}", q.size(), q.is_empty());
   println!("content:{:?}", q);

}