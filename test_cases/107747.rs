#![feature(trait_alias)]

use std::fmt;

trait Foo: fmt::Debug {}

// uncommenting the following line brings Debug into scope, which
// in turn causes the (unrelated) Display impl below to become ambiguous

// trait Bar = Foo;

#[derive(Debug)]
struct Qux(bool);

impl fmt::Display for Qux {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
self.0.fmt(f)
}
}

fn main() {}