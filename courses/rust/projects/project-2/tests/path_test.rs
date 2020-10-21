use std::env::current_dir;
use failure::_core::any::TypeId;
use std::path::{Path, PathBuf};
use std::fs;
use std::borrow::Borrow;

#[test]
fn test() {
    let path = current_dir().unwrap();
    println!("{:?}",path);
    let p:PathBuf = path.into();
    let pp:&Path = p.borrow();
    println!("{:?}",pp);

    fs::create_dir_all(p);
    let s = r"anc";
}