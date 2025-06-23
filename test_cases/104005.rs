use std::fmt::Display;

trait Displayable {
fn display(self) -> Box<dyn Display>;
}

impl<T: Display> Displayable for (T, Option<&'static T>) {
fn display(self) -> Box<dyn Display> {
Box::new(self.0)
}
}

fn extend_lt<T, U>(val: T) -> Box<dyn Display>
where
(T, Option<U>): Displayable,
{
Displayable::display((val, None))
}

fn main() {
// The type parameter `U = &'static &'temporary str` is ill-formed
// because it does not enforce `'temporary: 'static` ...
let val = extend_lt(&String::from("blah blah blah"));
println!("{}", val);
}