trait Trait {
const N: usize;
}

impl Trait for i32 {}

fn f()
where
[(); <i32 as Trait>::N]:,
{}

fn main() {}