
use std::str::Utf8Error;

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode, percent_decode};

use url::form_urlencoded::{byte_serialize, parse};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'=');

fn main() -> Result<(), Utf8Error> {
    let input = "confident, productive syntems programming = 1111";
    let iter = utf8_percent_encode(input, FRAGMENT);
    let encode: String = iter.collect();
    println!("{}", encode);

    let iter = percent_decode(encode.as_bytes());
    let decode = iter.decode_utf8()?;
    println!("{}", decode);


    urldecode();


    Ok(())

}

fn urldecode() {
    let urlencoded = byte_serialize("What is ❤?".as_bytes()).collect::<String>();
    println!("urldecoded: {:?}", urlencoded);

    let decoded = parse(urlencoded.as_bytes())
        .map(|(key, value)| { [key, value].concat()})
        .collect::<String>();
    println!("decoded: {}", decoded);

}