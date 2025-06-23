pub fn example() {
let a = 1;
let b = 16;

foo(a + b); // break here
}

#[inline(never)]
fn foo(x: i32) {
std::process::exit(x);
}

fn main() {}