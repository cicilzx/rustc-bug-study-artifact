#![feature(rustc_attrs)]
#![allow(internal_features)]

#[rustc_dump_vtable]
trait Marker {}
trait Pencil { fn f(&self) {} }

#[rustc_dump_vtable]
trait Imperfection: Pencil + Marker {}

struct T;
impl Marker for T {}
impl Pencil for T {}
impl Imperfection for T {}

fn main() {
(&T as &dyn Imperfection).f();
let _a = &T as &dyn Marker;
}