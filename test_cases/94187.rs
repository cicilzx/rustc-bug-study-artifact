#![feature(adt_const_params)]
#![feature(const_type_name)]
#![allow(incomplete_features)]

pub trait True {}

pub struct Equal<const A: &'static str, const B: &'static str>;

impl<const A: &'static str> True for Equal<{A}, {A}> {}

pub fn assert<T: True>(_: T) {}

fn main() {
assert(Equal::<{std::any::type_name::<[u32; 0]>()}, "[u32; 0]">);
}