

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

}