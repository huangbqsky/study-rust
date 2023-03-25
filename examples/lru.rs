#![allow(dead_code, unused)]

use std::collections::HashMap;
use std::hash::Hash;
const CHACHE_SIZE: usize = 100;

// LRU 上的数据项
struct Entry<K, V> {
    key: K,
    val: Option<V>,
    next: Option<usize>,
    prev: Option<usize>,
}

// LRU 缓存
struct LRUCache<K, V> {
    cap: usize,
    head: Option<usize>,
    tail: Option<usize>,
    map: HashMap<K, usize>,
    entries: Vec<Entry<K, V>>,

}
impl<K: Clone + Hash + Eq, V> LRUCache<K, V> {
   fn new() -> Self {
      Self::with_capacity(CHACHE_SIZE)
   }

   fn with_capacity(capacity: usize) -> Self {
      LRUCache { 
           cap: capacity, 
           head: None, 
           tail: None, 
           map: HashMap::with_capacity(capacity), 
           entries: Vec::with_capacity(capacity) 
        }
   }

   fn insert(&mut self, key: K, val: V) -> Option<V> {
      if self.map.contains_key(&key) {
        self.access(&key);
        let entry = &mut self.entries[self.head.unwrap()];
        let old_value = entry.val.take();
        entry.val = Some(val);
        old_value
      } else {
        self.ensure_room();
        let index = self.entries.len();
        self.head.map(|e| {
            self.entries[index].prev = Some(index);
        });

        self.entries.push(Entry{
            key: key.clone(),
            val: Some(val),
            prev: None,
            next: self.head,
        });
        self.head = Some(index);
        self.tail = self.tail.or(self.head);
        self.map.insert(key, index);
        None
      }
   }

   fn get(&mut self, key: &K) -> Option<&V> {
      if self.contains(key) {
          self.access(key);
      }
      let entries = &self.entries;
      self.map.get(key).and_then(move | i| {
          entries[*i].val.as_ref()
      })
   }
   fn get_mut(&mut self, key: &K) -> Option<&mut V> {
      if self.contains(key) {
         self.access(key);
      }
      let entries = &mut self.entries;
      self.map.get(key).and_then(move |i| {
        entries[*i].val.as_mut()
      })

   }

   fn contains(&mut self, key: &K) -> bool {
      self.map.contains_key(key)
   }

   fn ensure_room(&mut self) {
     if self.cap == self.len() {
        self.remove_tail();
     }
   }
   fn remove_tail(&mut self) {
      if let Some(index) = self.tail {
        self.remove_from_list(index);
        let key = &self.entries[index].key;
        self.map.remove(key);
      }
      if self.tail.is_none() {
        self.head = None;
      }
   }
   fn access(&mut self, key: &K) {
      let i = *self.map.get(key).unwrap();
      self.remove_from_list(i);
      self.head = Some(i);
   }
   fn remove(&mut self, key: &K) -> Option<V>{
      self.map.remove(&key).map(|index| {
        self.remove_from_list(index);
        self.entries[index].val.take().unwrap()
      })
   }
   fn remove_from_list(&mut self, i: usize) {
      let (prev, next) = {
        let entry = self.entries.get_mut(i).unwrap();
        (entry.prev, entry.next)
      };
      match (prev, next) {
        (Some(j), Some(k)) => {
            let head = &mut self.entries[j];
            head.next = next;
            let next = &mut self.entries[k];
            next.prev = prev;
        },
        (Some(j), None) => {
            let head = &mut self.entries[j];
            head.next = None;
            self.tail = prev;

        },
        _ => {
            if self.len() > 1 {
                let head = &mut self.entries[0];
                head.next = None;
                let next = &mut self.entries[1];
                next.prev = None;
            }

        }
      }
   }

   fn len(&self) -> usize {
     self.map.len()
   }
   fn is_empty(&self) -> bool {
      self.map.is_empty()
   }
   fn is_full(&self) -> bool {
      self.map.len() == self.cap
   } 

}

fn main() {

}