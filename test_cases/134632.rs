#![allow(unused)]
#![feature(negative_impls)]

trait MyTrait {}

#[derive(Clone)]
struct MyString {
string: String,
}

impl<T: Copy> !MyTrait for T { }

// Works
impl MyTrait for MyString { }
// Throws error but it should work
impl MyTrait for String { }

fn main() {

}