#![feature(unsized_fn_params)]
use std::any::Any;
use std::hint::black_box;

#[repr(align(1024))]
#[allow(dead_code)]
struct A(u8);

impl A {
fn f(&self) {
assert_eq!(0, black_box(self as *const A as usize) % 1024);
}
}

#[inline(always)]
pub fn f(a: dyn Any) {
a.downcast_ref::<A>().unwrap().f()
}

pub fn main() {
let a = Box::new(A(0)) as Box<dyn Any>;
f(*a);
}