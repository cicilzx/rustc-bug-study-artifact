#![crate_type = "lib"]

trait Q {
const ASSOC: usize;
}

impl<const N: u64> Q for [u8; N] {}

pub fn q_user() -> [u8; <[u8; 13] as Q>::ASSOC] {}

fn main() {}