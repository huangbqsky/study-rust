#![allow(dead_code, unused_imports)]
use clap::{Arg, App};
use ansi_term::{Style, Colour};

pub fn main_run(){
    let matches = App::new("My test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hackerman@hackerman.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .takes_value(true)
             .help("A cool file"))
        .arg(Arg::with_name("num")
             .short("num")
             .long("number")
             .takes_value(true)
             .help("Five less than your favorite number"))
        .get_matches();
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", Colour::Yellow.bold().paint(myfile));

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}", Colour::Red.bold().paint((n + 5).to_string())),
                Err(_) => println!("That's not a number {}", s),
                
            }
        }
    }
}