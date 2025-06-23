pub fn foo(t: &mut Vec<usize>) {
let mut taken = std::mem::take(t);
taken.pop();
*t = taken;
}
fn main() {}