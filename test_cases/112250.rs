trait Trait {
type Ty<'a> where Self: 'a;
}

impl<T> Trait for T {
type Ty<'a> = () where Self: 'a;
}

struct Foo<T: Trait>(T)
where
for<'a> T::Ty<'a>: Sized;

// implied bound: 'static: placeholder('a)
fn test(_: Foo<&'static str>) {}

fn main() {}