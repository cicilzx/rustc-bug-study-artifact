trait Supertrait<'a, 'b> {
fn cast(&self, x: &'a str) -> &'b str;
}

trait Subtrait<'a, 'b> {}

impl<'a> Subtrait<'a, 'a> for () {}
fn unsound(x: &dyn for<'a> Subtrait<'a, 'a>) -> &dyn for<'a, 'b> Supertrait<'a, 'b> {
x
}

fn transmute<'a, 'b>(x: &'a str) -> &'b str {
unsound(&()).cast(x)
}

fn main() {
let x;
}