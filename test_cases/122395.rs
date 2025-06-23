#![feature(generic_const_exprs)]

fn bb<const N: bool>() {}

fn b<const N: bool>() {
bb::<{!N}>();
}

fn main() {}