#![feature(convert, path_ext)]

extern crate flate2;
use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::fs::{File};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let path = PathBuf::from("test.gz");
    match fast_gunzip_file(&path) {
        Ok(bytes) => {
            println!("{}", std::str::from_utf8(&bytes).unwrap());
        },
        Err(e) => {
            println!("IO error reading/ungzipping filename={} error={}",
                     &path.to_str().unwrap(), e);
        }
    };
    match slow_gunzip_file(&path) {
        Ok(bytes) => {
            println!("{}", std::str::from_utf8(&bytes).unwrap());
        },
        Err(e) => {
            println!("IO error reading/ungzipping filename={} error={}",
                     &path.to_str().unwrap(), e);
        }
    };
}

fn fast_gunzip_file(filename: &PathBuf) -> std::io::Result<Vec<u8>> {
    println!("fast gunzip filename={}", &filename.to_str().unwrap());
    let size = match filename.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(e) => return Err(e)
    };

    let gz_file = try!(File::open(&filename));
    let mut decoder = try!(GzDecoder::new(gz_file));
    let mut bytes = Vec::new();
    try!(decoder.read_to_end(&mut bytes));
    println!("unzipped");
    Ok(bytes)
}

fn slow_gunzip_file(filename: &PathBuf) -> std::io::Result<Vec<u8>> {
    println!("slow gunzip filename={}", &filename.to_str().unwrap());
    let size = match filename.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(e) => return Err(e)
    };

    let gz_file = try!(File::open(&filename));
    let mut decoder = try!(GzDecoder::new(gz_file));
    let mut bytes = Vec::with_capacity(200_000_000);
    try!(decoder.read_to_end(&mut bytes));
    println!("unzipped");
    Ok(bytes)
}
