#![allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Persion {
    name: String,
    age: u32,
}
impl Persion {
    pub fn new(name: String, age: u32) -> Self  { 
       Persion { name: name, age: age }
    }
    
}

pub fn main_run() {
    let mut vec = vec![1, 5, 10, 2, 15];
    
    vec.sort();
    println!("u32 vec = {:?}", vec);
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);


    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    // vec.sort();
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("f64 vec = {:?}", vec);

    // struce sort
    let mut people = vec![
        Persion::new("Zeo".to_string(), 25),
        Persion::new("Aladdin".to_string(), 60), 
        Persion::new("John".to_string(), 1)
    ];
    people.sort();
    println!("sort people= {:?}", people);
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("sort_by people= {:?}", people);
}
