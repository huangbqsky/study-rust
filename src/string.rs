

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

    // 连接: 
    // 1. 使用 + 或者 += 连接字符串, 要求右边的参数必须为字符串的切片引用（Slice)类型
    // 其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法，这里 add() 方法的第二个参数是一个引用的类型。
    // 因此我们在使用 +， 必须传递切片引用类型。不能直接传递 String 类型。
    // + 和 += 都是返回一个新的字符串。所以变量声明可以不需要 mut 关键字修饰

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    // 2. 使用 format! 连接字符串,format! 这种方式适用于 String 和 &str 。
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("format格式化字符串：{}", s);

    // 字符串转义， 通过转义的方式 \ 输出 ASCII 和 Unicode 字符。
      // 通过 \ + 字符的十六进制表示，转义输出一个字符，\\保持字符串原样；\3F -> ？
      let byte_escape = "I'm writing \x52\x75\x73\x74!";
      println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
  
      // \u 可以输出一个 unicode 字符
      let unicode_codepoint = "\u{211D}";
      let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
  
      println!(
          "字符串转义：Unicode character {} (U+211D) is called {}",
          unicode_codepoint, character_name
      );
  
      // 换行了也会保持之前的字符串格式
      let long_string = "String literals
                          can span multiple lines.
                          The linebreak and indentation here ->\
                          <- can be escaped too!";
      println!("保持之前的字符串格式： {}", long_string);

      // 字符： 如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法
      for c in "中国人".chars() {
        println!("{}", c);
      }

      // 字节：这种方式是返回字符串的底层字节数组表现形式
      for b in "中国人".bytes() {
        println!("{}", b);
      }

      ///总结：
      // 1。字符串切片索引要注意，必须落在字符边界
      // 2. push，insert，replace_range， pop，remove，truncate，clear 修改原有字符串，即字符串必须是 mut
      // 3. replace，replacen，catenate, format!返回新的字符串，不用 mut。
}