use std::collections::HashMap;

use lazy_static::lazy_static;

// 使用 lazy_static 声明一个在运行期初始化( 懒求值 )的 Hashmap，它会被求值一次，然后保存在一个全局的 static 引用之后。
lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jims", vec!["user"]);
        map
    };
}

fn show_access(name: &str) { 
    let access = PRIVILEGES.get(name);
    println!("{}, {:?}", name, access);

}
fn main() { 
    let access = PRIVILEGES.get("James");
    println!("James: {:?}", access);

    show_access("Jims");

}