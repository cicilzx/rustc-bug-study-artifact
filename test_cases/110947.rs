#![feature(custom_mir, core_intrinsics)]
extern crate core;
use core::intrinsics::mir::*;
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0() -> bool {
mir! {
type RET = bool;
let pair: (i8, bool);
let ptr: *mut bool;
{
pair = (1, false);
ptr = core::ptr::addr_of_mut!(pair.1);
RET = pair.1 <= (*ptr);
pair = (1, false);
(*ptr) = RET | RET;
RET = !pair.1;
Return()
}

}
}
pub fn main() {
println!("{}", fn0());
}