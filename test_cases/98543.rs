trait Trait {
type Type;
}

impl<T> Trait for T {
type Type = ();
}

fn f<'a, 'b>(s: &'b str, _: <&'a &'b () as Trait>::Type) -> &'a str
where
for<'what, 'ever> &'what &'ever (): Trait,
{
s
}

fn main() {
let x = String::from("Hello World!");
let y = f(&x, ());
drop(x);
println!("{}", y);
}