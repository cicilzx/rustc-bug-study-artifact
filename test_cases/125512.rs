#![feature(object_safe_for_dispatch)]
trait B {
fn f(a: A) -> A;
}
trait A {
fn concrete(b: B) -> B;
}
fn main() {}