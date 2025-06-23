#![feature(custom_mir, core_intrinsics)]
extern crate core;
use core::intrinsics::mir::*;

#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6() {
mir! {
let _10: (Adt19,);
let arr: [u32;8];
let _23: *mut (Adt19,);
let _29: i128;
let _57: (u64, i128, char, isize);
let _72: isize;
let _90: Adt19;
let _91: [char; 2];
let _92: Adt19;
let _93: Adt19;
let _102: ();
{
_57 = (0, 0, 'A', 0);
place!(Field::<(u64, i128, char, isize)>(Variant(_10.0, 3), 1)) = _57;
_93 = Adt19::Variant3 { fld0: false,fld1: Field::<(u64, i128, char, isize)>(Variant(_10.0, 3), 1),fld2: 0,fld3: 0.,fld4: 0.,fld5: 0,fld6: 0};
place!(Field::<(u64, i128, char, isize)>(Variant(_93, 3), 1)).2 = 'A';
_23 = core::ptr::addr_of_mut!(_10);
_29 = Field::<(u64, i128, char, isize)>(Variant(_93, 3), 1).1;
_23 = core::ptr::addr_of_mut!(_10);
place!(Field::<(u64, i128, char, isize)>(Variant(_93, 1), 2)) = (0, 0, 'A', 0);
place!(Field::<(u64, i128, char, isize)>(Variant(_10.0, 3), 1)) = Field::<(u64, i128, char, isize)>(Variant(_93, 1), 2);
arr = [0;8];
_93 = Adt19::Variant2 { fld0: 0,fld1: 0,fld2: 0.,fld3: 0,fld4: arr,fld5: _57 };
(*_23) = (_93,);
_92 = _10.0;
place!(Field::<(u64, i128, char, isize)>(Variant((*_23).0, 2), 5)) = (Field::<(u64, i128, char, isize)>(Variant(_92, 2), 5).0, 0, 'A', 0);
_72 = Field::<(u64, i128, char, isize)>(Variant(_92, 2), 5).3;
_90 = Adt19::Variant3 { fld0: false,fld1: Field::<(u64, i128, char, isize)>(Variant(_10.0, 2), 5),fld2: Field::<(u64, i128, char, isize)>(Variant(_10.0, 2), 5).3,fld3: 0.,fld4: 0.,fld5: 0,fld6: 0 };
_10.0 = Adt19::Variant1 { fld0: 0.,fld1: 0,fld2: Field::<(u64, i128, char, isize)>(Variant(_90, 3), 1),fld3: 0};
_91 = [Field::<(u64, i128, char, isize)>(Variant((*_23).0, 1), 2).2,'A'];
Goto(bb53)
}
bb53 = {
Call(_102 = p(_91), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_102 = black_box(Move(_57)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}
}
}
pub fn main() {
fn6();
}
#[derive(Debug, Copy, Clone)]
pub enum Adt19 {
Variant0 {},
Variant1 {
fld0: f32,
fld1: u8,
fld2: (u64, i128, char, isize),
fld3: i128,
},
Variant2 {
fld0: u32,
fld1: u8,
fld2: f64,
fld3: u16,
fld4: [u32; 8],
fld5: (u64, i128, char, isize),
},
Variant3 {
fld0: bool,
fld1: (u64, i128, char, isize),
fld2: isize,
fld3: f32,
fld4: f64,
fld5: usize,
fld6: i64,
},
}
fn black_box(x: (u64, i128, char, isize)) {
core::hint::black_box(x);
}

#[cfg(not(miri))]
fn print_u32(x: u32) {
extern "C" {
fn printf(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

unsafe {
printf(b"%d\n\0".as_ptr().cast(), x);
}
}

#[cfg(miri)]
fn print_u32(x: u32) {
println!("{x}");
}
#[inline(never)]
fn p(x: [char; 2]) {
print_u32(x[0] as u32)
}