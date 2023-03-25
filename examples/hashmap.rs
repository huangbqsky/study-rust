#![allow(dead_code)]

// 用slot保存位置，data保存数据
#[derive(Debug, Clone, PartialEq)]
struct HashMap<T> {
    size: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}
impl<T: Clone + PartialEq + Default> HashMap<T> {
    fn new(size: usize) -> Self {
        // 初始化slot和data
        let mut slot = Vec::with_capacity(size);
        let mut data = Vec::with_capacity(size);
        for _i in 0..size {
            slot.push(0);
            data.push(Default::default());
        }
        HashMap { size, slot, data }
    }
    fn hash(&self, key: usize) -> usize {
        key % self.size
    }
    fn rehash(&self, pos: usize) -> usize {
       (pos + 1) % self.size
    }

    fn insert(&mut self, key: usize, value: T) {
        if 0 == key { panic!("Error: key must > 0"); }
        let pos = self.hash(key);
        if 0 == self.slot[pos] { //卡槽无数据直接插入
           self.slot[pos] = key;
           self.data[pos] = value;
        } else { // 插入槽有数据则找下一个可行的位置
            let mut next = self.rehash(pos);
            while 0 != self.slot[next] && key != self.slot[next] {
                next = self.rehash(next);
                if next == pos { // 槽满了就去退出
                  println!("Error: slot is full, quit insertion");
                  return;
                }
            }

            // 在找到的槽插入数据
            if 0 == self.slot[next] {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key { panic!("Error: key must > 0");}

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            None
        } else if key == self.slot[pos] {
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data: Option<T> = None;
            let mut stop = false; let mut found = false;
            let mut curr = pos;
            while 0 != self.slot[curr]  && !found && !stop {
                if key == self.slot[curr] { // 找到值，删除
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }
            data
        }
    }

    fn get(&self, key: usize) -> Option<&T> {
        if 0 == key { panic!("Error: key must > 0");}
        let pos = self.hash(key);
        let mut data: Option<&T> = None;
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;
        // 循环查找数据
        while 0 != self.slot[curr] && !found  && !stop { 
            if key == self.slot[curr] {
                found = true;
                data = self.data.get(curr);
            } else {
                // 再哈希回到初始位置，说明找了一圈没找到
                curr  = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }
        data
    }

    fn contains(&self, key: usize) -> bool { 
        if 0 == key { panic!("Error: key must > 0");}
        self.slot.contains(&key)
    }

    fn len(&self) -> usize { 
        let mut length = 0;
        for d in self.slot.iter() {
            if &0 != d { // 槽数据不为空，表述有数据，len +1
                length += 1;
            }
        }

        length
    }


}
fn main() {

    let mut hmap = HashMap::new(11);
    hmap.insert(10, "cat");
    hmap.insert(2, "dog");
    hmap.insert(3, "tiger");

    println!("HashMap size: {}", hmap.len());
    println!("HashMap contains key 2: {}", hmap.contains(2));
    println!("HashMap key 3: {:?}", hmap.get(3));
    println!("HashMap remove key 3: {:?}", hmap.remove(3));
    println!("HashMap remove key 3: {:?}", hmap.remove(3));

}