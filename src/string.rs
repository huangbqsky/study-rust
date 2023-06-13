

pub fn main() {

    let mut s = String::from("Hello");

    s.push('c');

    let ps = s.as_str();

    unsafe {

        let (valps1, valps2) : (usize, usize) = std::mem::transmute(ps);

        let (vals1, vals2, vals3): (usize, usize, usize) = std::mem::transmute(s);

        println!("&str 0x{:x} {}", valps1, valps2);

        println!("String 0x{:x} {} {}", vals1, vals2, vals3);

    }

    // 替换 replace 该方法可适用于 String 和 &str 类型， 返回一个新的字符串
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    // 替换 replacen  返回一个新的字符串
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);

    // 替换 replace_range，仅适用于 String 类型
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // 删除：与字符串删除相关的方法有 4 个，分别是 pop()，remove()，truncate()，clear()。这四个方法仅适用于 String 类型。
    // 1. pop —— 删除并返回字符串的最后一个字符，该方法是直接操作原来的字符串。
    // 但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    // 2. remove —— 删除并返回字符串中指定位置的字符， 该方法是直接操作原来的字符串。但是存在返回值，其返回值是删除位置的字符串
    // remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    // 3。truncate —— 删除字符串中从指定位置开始到结尾的全部字符， 该方法是直接操作原来的字符串。无返回值。
    // 该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    //4。clear —— 清空字符串， 该方法是直接操作原来的字符串。
    // 调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

}