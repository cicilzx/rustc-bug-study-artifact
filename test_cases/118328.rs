#![feature(custom_mir, core_intrinsics, inline_const)]
extern crate core;
use core::intrinsics::mir::*;

#[custom_mir(dialect = "runtime", phase = "initial")]
fn size_of<T>() -> usize {
mir! {
let a : usize;
{
a = 0;
a = const { std::mem::size_of::<T>() };
RET = a;
Return()
}
}
}

fn main() {
assert_eq!(size_of::<u32>(), std::mem::size_of::<u32>());
}