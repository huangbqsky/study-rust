#![allow(dead_code)]
fn main() {
    let mut sharps: Vec<&dyn Area> = vec![];
    sharps.push(&Square(3.0));
    sharps.push(&Rectangle(3.0, 2.0));

    println!("{}", sharps[0].get_area());
    println!("{}", sharps[1].get_area());
}

trait Area {
    fn get_area(&self) -> f64;
}

struct Square(f64);
struct Rectangle(f64, f64);

impl Area for Square {
    fn get_area(&self) -> f64 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f64 {
        self.0 * self.1
    }
}


trait Fly {
    fn fly(&self);
}
// warning: trait objects without an explicit `dyn` are deprecated
fn dynamic_fly(fly: &dyn Fly) {
    fly.fly()
}
fn static_fly(fly: impl Fly) {
    fly.fly()
}

fn dynamic_fly1(fly: Box<dyn Fly>) {
    fly.fly()
}

// 使用 Box<dyn Clone> 或 &dyn Clone 会报错
// fn dynamic_fly2(text: &dyn Clone) { // Clone` cannot be made into an object
//   text.clone()
// }
