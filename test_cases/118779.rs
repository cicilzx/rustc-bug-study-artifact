pub trait Add {
fn add(&self, left: usize, right: usize) -> usize;
}

impl<F> Add for F
where
F: Fn(usize, usize) -> usize,
{
fn add(&self, left: usize, right: usize) -> usize {
self(left, right)
}
}

pub struct NotAdd;

pub fn needs_add<A: Add>(add: &A) -> usize {
add.add(1, 2)
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn it_works() {
assert_eq!(needs_add(&NotAdd), 3);
}
}

fn main() {}