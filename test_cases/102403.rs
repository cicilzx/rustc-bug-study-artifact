#![feature(const_fn_floating_point_arithmetic)]

const fn f(one: f64) -> f64 {
(-1.0) % one
}

const RESULT_CT : f64 = f(1.0);

fn main() {
let black_box_one = (std::env::args().len()) as f64;
let result_rt = f(black_box_one);
assert_eq!(RESULT_CT.is_sign_negative(), result_rt.is_sign_negative());
}