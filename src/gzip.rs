#![allow(dead_code)]
use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use tar::Archive;

#[allow(unused_variables)]
pub fn main_run() {
    let zip = gzip();
    let unzip = unzip();
}

fn gzip() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}

fn unzip() -> Result<(), std::io::Error> {
    let tar_gz = File::open("archive.tar.gz")?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

fn zip_filter() -> Result<()>{
    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "bundle/logs";
    println!("Extracted the followding files:");
    archive
        .entries()?
        .filter_map(|e| e. ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)

        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}