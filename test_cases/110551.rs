pub fn f(a: Option<u32>) -> u32 {
match a {
None => {}
Some(_) => unsafe { std::hint::unreachable_unchecked() }
}

a.unwrap()
}

fn main() {}