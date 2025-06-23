#![warn(explicit_outlives_requirements)]
#![allow(dead_code)]

macro_rules! make_baz {
($a:lifetime, $b:lifetime) => {
struct Baz<$a, $b: $a> where (): Sized, $b: $a {
baz: &$a &$b (),
}
};
}

make_baz!{ 'a, 'b }

fn main() {}