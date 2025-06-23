use inari::{Interval as I, const_interval};

fn f(u: I) -> f64 {
let t = u.inf() == 0.;
// let t = u.inf() == -0.;
// println!("{}", t);
if t { 1. }
else { println!("{} {}", u.inf(), u.inf() == 0.);
u.inf() }
}

fn main() {
println!("{}", f(const_interval!(0., 0.)));
}