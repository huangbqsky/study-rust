#![allow(dead_code)]

// 字典树定义
#[derive(Default, Debug)]
struct Trie {
    root: Node,
}

// 节点
#[derive(Default, Debug)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26], // 26个字符列表
}
impl Trie {
    fn new () -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &str){
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            let next = &mut node.children[index];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.end = true;

    }

    fn search(&mut self, word: &str) -> bool {
        self.word_node(word).map_or(false, |n| n.end)
    }

    fn start_with(&self, prefix: &str) -> bool{
        self.word_node(prefix).is_some()
    }

    fn word_node(&self, wps: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in wps.as_bytes() {
            let index = (c - b'a') as usize;
            match &node.children[index] {
                None => return None,
                Some(next) => node = next.as_ref(),
            }  
        }
        Some(node)
    }

}
fn main() {
    let mut trie = Trie::new();
    trie.insert("box"); trie.insert("insert");
    trie.insert("apple"); trie.insert("appeal");
    trie.insert("kkk");

    let res1 = trie.search("apple");
    let res2 = trie.search("apples");
    let res3 = trie.search("ins");
    let res4 = trie.search("ina");
    let res5 = trie.search("888");

    println!("word 'apple' in Trie: {res1}");
    println!("word 'apples' in Trie: {res2}");
    println!("prefix 'ins' in Trie: {res3}");
    println!("prefix 'ina' in Trie: {res4}");
    println!("prefix '888' in Trie: {res5}");

    println!("Trie: {:#?}", trie);

}