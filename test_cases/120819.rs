#![feature(custom_mir, core_intrinsics)]
#![allow(unused_parens, unused_assignments, overflowing_literals)]
extern crate core;
use core::intrinsics::mir::*;

pub fn enter() {
fn13(core::hint::black_box((0, [false; 6], [0; 4], 0, 0)));
}

#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _6: (i8, [bool; 6], [i64; 4], i128, u8)) {
mir! {
let _1: [bool; 6];
let _14: [u16; 6];
let _17: *const (i8, [bool; 6], [i64; 4], i128, u8);
let _19: Adt59;
let _27: (i8, [bool; 6], [i64; 4], i128, u8);
let _34: Adt54;
let _49: ();
{
_1 = [false,true,true,true,true,true];
_19.fld4 = [25264_u16,54664_u16,47032_u16,55453_u16,33511_u16,43043_u16];
_17 = core::ptr::addr_of!(_6);
_1 = (*_17).1;
(*_17).1 = [false,true,true,false,false,true];
Call(_49 = fn14(_6.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_17).1 = _1;
_14 = _19.fld4;
_17 = core::ptr::addr_of!(_27);
_34.fld5 = (-1228700359_i32);
_34.fld1 = _6;
match _34.fld5 {
0 => bb3,
340282366920938463463374607430539511097 => bb12,
_ => bb18
}
}
bb12 = {
(*_17) = (_6.0, _6.1, _6.2, _34.fld1.3, _6.4);
_1 = _27.1;
Call(_49 = p1(Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = hide(13_usize, 25_usize, 0_usize, 11_usize, 0_usize, 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}

#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _16: [bool; 6]) {
mir! {
let _20: (i8, [bool; 6], [i64; 4], i128, u8);
let unit: ();
let _44: ();
{
Call(_44 = core::hint::black_box(unit), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20.1 = core::hint::black_box(Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
enter();
}

#[derive(Debug)]
pub struct Adt54 {
fld1: (i8, [bool; 6], [i64; 4], i128, u8),
fld5: i32,
}
#[derive(Debug)]
pub struct Adt59 {
fld4: [u16; 6],
}

extern "C" {
fn printf(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
#[cfg(not(miri))]
fn print_bool(x: bool) {
unsafe {
printf(b"%d\n\0".as_ptr().cast(), x as core::ffi::c_int);
}
}

#[cfg(miri)]
fn print_bool(x: bool) {
println!("{}", x as i32);
}

#[inline(never)]
pub fn p1(x: [bool; 6]) {
for b in x {
print_bool(b);
}
}

#[inline(never)]
fn hide(
f: usize,
var0: usize,
val0: usize,
var1: usize,
val1: usize,
var2: usize,
val2: [u16; 6],
var3: usize,
val3: (i8, [bool; 6], [i64; 4], i128, u8),
) {
core::hint::black_box(val2);
core::hint::black_box(val3);
}