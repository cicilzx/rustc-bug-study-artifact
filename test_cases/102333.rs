#![crate_type = "lib"]

trait A {
type T: B<U<1i32> = ()>;
}

trait B {
type U<const C: i32>;
}

fn main() {}