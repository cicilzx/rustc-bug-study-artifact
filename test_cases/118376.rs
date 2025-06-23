struct Foo<'a>(&'a ());

impl<'a> Foo<'a> {
fn bar<'b: 'a>(&'b self) {} // WARN: `'a` and `'b` are equal
}

fn main() {}