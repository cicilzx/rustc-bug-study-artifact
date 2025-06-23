#![feature(custom_mir, core_intrinsics)]
extern crate core;
use core::intrinsics::mir::*;

pub fn dump_var(val0: u32) {
println!("{val0}");
}

pub struct Adt52 {
fld1: (u32, usize, u16),
}

#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14() {
mir! {
let fld1: (u32, usize, u16);
let non_copy: Adt52;
let p: *const u32;
let i: u32;
let unit: ();
{
fld1 = (0, 0_usize, 0);
non_copy = Adt52 {fld1};
p = core::ptr::addr_of!(non_copy.fld1.0);
Call(unit, bb13, fn15(Move(non_copy)))
}
bb13 = {
i = *p;
Call(unit, bb18, dump_var(i))
}
bb18 = {
Return()
}

}
}
pub fn fn15(mut x: Adt52) {
x.fld1 = (1, 0, 0);
}
pub fn main() {
fn14();
}