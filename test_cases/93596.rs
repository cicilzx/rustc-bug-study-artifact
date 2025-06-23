use std::ffi::{OsStr, OsString};
use std::path::Path;

fn check(p: &dyn AsRef<Path>) {
let m = std::fs::metadata(&p);
println!("{:?}", &m);
}

fn main() {
let s: OsString = ".".into();
let s: &OsStr = &s;
check(s);
}