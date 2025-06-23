#![feature(auto_traits)]

auto trait Trait {
type Output;
}

fn f() {
let _: <i32 as Trait>::Output = 1_i64;
}

fn main() {}