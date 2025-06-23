trait A<T>: B<T = T> {}

trait B {
type T;
}

struct Erase<T: ?Sized + B>(T::T);

fn main() {
let x = {
let x = String::from("hello");

Erase::<dyn for<'a> A<&'a _>>(x.as_str())
};

dbg!(x.0);
}