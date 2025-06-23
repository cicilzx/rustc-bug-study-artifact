#![feature(stmt_expr_attributes)]
pub struct S([usize; 8]);

#[no_mangle]
pub fn f(x: S, y: S) -> usize {
(#[inline(always)]|| {
let _z = x;
y.0[0]
})()
}

fn main() {}