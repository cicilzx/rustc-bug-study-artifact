fn main() {
    type TAIT = impl for<'a> Foo<'a, Assoc = impl Sized>;
}