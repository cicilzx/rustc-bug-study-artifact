struct Data {
v: Vec<i32>,
}

impl Iterator for Data {
type Item = &[i32];
fn next(&mut self) -> Option<Self::Item> {
let mut a = 0;
let mut b = 0;
Some(&self.v[a..b])
}
}

fn main() {}