trait A<T> {}

trait B {
type Type;
}

impl<T> B for T
where
T: A<Self::Type>,
{
type Type = bool;
}

fn main() {}