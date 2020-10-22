use std::borrow::Borrow;
use std::env::current_dir;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::create_dir_all;
use std::io::Error;
use std::path::{Path, PathBuf};

use failure::_core::any::TypeId;

#[test]
fn test() {
    let path = current_dir().unwrap();
    println!("{:?}", path);
    let p: PathBuf = path.into();
    let pp: &Path = p.borrow();
    println!("{:?}", pp);

    fs::create_dir_all(p);
    let s = r"anc";
}

#[test]
fn test_create() {
    let mut path = PathBuf::new();
    path.push("/Users/liuchao56/d2");
    create_dir_all(path).unwrap();

    let mut path = Path::new("/Users/liuchao56");
    let r = fs::read_dir(path).unwrap();
    println!("{:?}", r)
}

#[test]
fn test_map() {
    let v = vec![1, 2, 3];
    let r = v.iter().map(|e| e + 1).collect::<Vec<i32>>();
    let words = ["alpha", "beta", "gamma"];
    let merged = words.iter()
        .flat_map(|s| s.chars())
        .collect::<String>();
    println!("{:?}", r);
    println!("{:?}", merged);
}

#[test]
fn test_file_map() {
    let mut path = PathBuf::new();
    path.push("/Users/liuchao56/");
    let r = fs::read_dir(path).unwrap().flat_map(|dir| -> Result<PathBuf, Error> {
        //println!("{:?}",dir);
        Ok(dir?.path())
    }).map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<String>();
    println!("{:?}", r)
}

#[test]
fn test_file_map2() {
    let mut path = PathBuf::new();
    path.push("/Users/liuchao56/");
    let r = fs::read_dir(path).unwrap();
    println!("{:?}", r)
}