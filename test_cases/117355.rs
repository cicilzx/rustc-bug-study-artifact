#![feature(custom_mir, core_intrinsics)]
#![allow(unused_parens, unused_assignments, overflowing_literals)]
extern crate core;
use core::intrinsics::mir::*;

#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(
mut _6: *mut bool,
mut _9: *mut bool,
) -> char {
mir! {
let _19: i8;
{
Goto(bb1)
}
bb1 = {
Call((*_9) = fn13(_6), bb2)
}
bb2 = {
_19 = 115_i8;
match _19 {
1 => bb3,
_ => bb5
}
}
bb3 = {
Call((*_9) = fn13(_6), bb2)
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *mut bool) -> bool {
mir! {
{
RET = true;
Return()
}
}
}
pub fn main() {}