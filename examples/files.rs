#![allow(unused_must_use, dead_code)]
use anyhow::{Result as AResult, Ok};

use std::collections::HashMap;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() -> AResult<()> {
    let path = "lines.txt";
    let mut file;
    if !Path::new(path).exists() {
        file = File::create(&path)?;
        file.write("Rust\n@@22\nFun\n".as_bytes())?;
    } else {
        file = OpenOptions::new().append(true).open(path)?;
        file.write_all("append line\n".as_bytes())?;
    }

    let input = File::open(&path)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }

    // dir();
    // walk_dir();
    Ok(())
}

fn dir() -> AResult<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in: {:?}:",
        current_dir
    );
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();
        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds , is read only :{:?}, size:{:?} bytes, filename:{:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename").unwrap()
            );
        }
    }

    Ok(())
}

fn walk_dir() {
    let mut filenames = HashMap::new();
    // 遍历当前目录
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn walk_dir_filter() -> AResult<()> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let sec = entry.metadata()?.modified()?;

        if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 60 * 10000 {
            println!("{}", f_name);
        }
    }

    // 遍历目录跳过隐藏文件
    WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));

    // 递归计算给定深度的文件大小
    WalkDir::new(".")
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    Ok(())
}

/// 查找满足给定正则的所有文件且忽略文件名大小写
fn glob_dir() -> AResult<()> {
    // 递归查找所有 png 文件
    for entry in glob::glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    let options = glob::MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    // glob_with 函数可以按照给定的正则表达式进行查找，同时还能使用选项来控制一些匹配设置。
    for entry in glob::glob_with("/target/img_[0-9]*.png", options)? {
        println!("{}", entry?.display());
    }

    Ok(())
}


fn read_all_lines(filename: &str) -> std::io::Result<()>{
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        println!("{}", line);
    }

    core::result::Result::Ok(())
}
