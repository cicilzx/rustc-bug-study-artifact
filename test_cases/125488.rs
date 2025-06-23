struct S<'a>(&'a ());
fn f(s: S<'_>) -> _ {
s
}

fn main() {}