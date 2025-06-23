use crate_b::Bar;

fn bar<B: Bar>() {}

pub mod crate_b {
pub trait Bar: Super<SuperAssoc: Bound> {}

pub trait Super {
type SuperAssoc;
}

pub trait Bound {}
}

fn main() {}