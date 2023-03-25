use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标取值，并创建一个’vec‘的引用。
        let vec = &self.0;
        write!(f, "[")?;
        // 使用‘v’对‘vec’进行迭代，并且‘count’迭代记录次数
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let list = List(vec![1, 2, 3, 4, 5]);
        println!("{}", list);
    }
}
