#![allow(dead_code)]
mod array;
mod buffer;
mod closure;
mod enumeration;
mod format;
// mod iter;
mod list;
mod matchs;
mod mini_string;
mod parse;
mod size_of;
mod structure;
mod tuple;
mod types;
mod vtable;
mod traits;
mod phantom;
mod errors;
mod boxed;
mod paths;
mod files;
mod processes;
mod env;
mod ffi;
mod sort;
mod gzip;
mod digest;
mod claps;
mod stack;
mod string;

#[derive(Debug, Default)]
enum Kind {
    #[default]
    A,
    B,
    C,
}

fn main() {
    // let order1 = Kind::default();
    // println!("{:?}", order1);
    
    // size_of::show_size_fn();
    // println!("{}", "-".repeat(64));
    // vtable::vtest();
    // println!("{}", "-".repeat(64));
    // buffer::main_run();
    // println!("{}", "-".repeat(64));
    // format::main_fn();
    // println!("{}", "-".repeat(64));
    // tuple::main();
    // array::main();
    // structure::main();
    // enumeration::main();
    // types::main();
    // iter::main();
    // matchs::main();
    // closure::main();
    // traits::main();
    // phantom::main();
    // errors::main();
    // boxed::main();
    // paths::main();
    // files::main();
    // processes::main();
    // env::main();
    // ffi::main();
    // sort::main_run();
    // gzip::main_run();
    // digest::main_run();
    // claps::main_run();
    
    string::main();

}

