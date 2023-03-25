use stack::Stack;
mod stack;

fn num2str_stk(mut num: i32, base: i32) -> String { 
    let digits: [&str;  16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                               "A", "B", "C", "D", "E", "F"];
    let mut rem_stack = Stack::new();
    while num > 0 {
        if num < base {
            rem_stack.push(num);
        } else {
            rem_stack.push(num % base);
        }
        num /= base;
    }

    let mut numstr = "".to_string();
    while !rem_stack.is_empty() {
        numstr += digits[rem_stack.pop().unwrap() as usize];
    }
    numstr
}
fn main() {

    let num = 100;
    let sb = num2str_stk(100, 2);
    let so = num2str_stk(100, 8);
    let sh = num2str_stk(100,16);

    println!("{num} is b{sb}, o{so}, x{sh}");

}