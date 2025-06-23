#![feature(custom_mir, core_intrinsics)]
extern crate core;
use core::intrinsics::mir::*;

#[inline(never)]
fn dump_var(x: [u128; 6]) {
println!("{x:?}");
}

#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0() {
mir! {
let _12: usize;
let _14: [u128; 6];
let _26: ([u128; 6],);
let _29: ([u128; 6],);
let x: ();
{
_12 = 1_usize;
_14 = [42; 6];
_14[_12] = 1;
_29 = (_14,);
_26 = _29;
Call(x = fn1(_29.0, _26), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}
}
}

pub fn fn1(mut _13: [u128; 6], mut _14: ([u128; 6],)) {
_14.0 = [0; 6];
dump_var(_13);
dump_var(_14.0);
}
pub fn main() {
fn0();
}