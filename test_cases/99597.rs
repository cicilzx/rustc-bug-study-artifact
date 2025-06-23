trait T1 { }

trait T2 {
fn test(&self) { }
}

fn go(s: &impl T1) {
s.test();
}

fn main() {}