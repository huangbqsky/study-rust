#![allow(dead_code, unused_must_use)]

use std::fs::File;
use std::io::{Read, Write, BufReader};
use ring::digest::{Context, Digest, SHA256};
use data_encoding::HEXUPPER;
// use error_chain::error_chain;
use anyhow::Result;

use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

// error_chain!{
//     foreign_links{
//         Io(std::io::Error);
//         Decode(data_encoding::DecodeError);
//     }
// }

fn sha256_digest<R: Read>(mut reader : R) -> Result<Digest> {
   let mut context = Context::new(&SHA256);
   let mut buffer = [0; 1024];
   loop {
       let count = reader.read(&mut buffer)?;
       if count == 0 {
          break;
       }
       context.update(&buffer[..count]);
   }
   Ok(context.finish())
}

pub fn sha256() -> Result<()>{
   let path = "file.txt";
   let mut output  = File::create(path)?;
   write!(output, "We will generate a digest of this text")?;
   
   let input = File::open(path)?;
   let reader = BufReader::new(input);
   let digest = sha256_digest(reader)?;
   println!("SHA-256 digest is: {}", HEXUPPER.encode(digest.as_ref()));

   Ok(())
}

fn sha256_hmac() -> Result<(), Unspecified>{
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "We will generate a digest of this text";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
    println!("SHA-256 hmac signature is {:?}", signature);

    Ok(())
}

pub fn main_run() {
    sha256();
    sha256_hmac();
    println!("{}", "-".repeat(64));
}