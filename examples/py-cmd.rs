// use anyhow::Result;
use std::process::{Command, Stdio};
use std::io::Write;
use std::collections::HashSet;
use error_chain::error_chain;

error_chain!{
    errors { CmdError }
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
    }
}

fn main() -> Result<()> { 
    println!("------------py-commond-------------------------");
    let mut child = Command::new("python3")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin
        .as_mut()
        .ok_or("Child process stdio has not been captured")?
        .write_all(b"import this; copyright (); credits; exit()")?;
    let output = child.wait_with_output()?;
    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
        Ok(())
    } else {
       let err = String::from_utf8(output.stdout)?;
       eprintln!("err= {:#?}", err);
       error_chain::bail!("External commond failed:\n {}", err);
    }
}