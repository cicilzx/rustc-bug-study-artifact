#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

macro_rules! co_vec_default {
() => {
1
}
}

struct V<const U: usize>
where
[(); U]:;

trait Tr {}

impl Tr for V<{co_vec_default!()}> {}

fn main() {}