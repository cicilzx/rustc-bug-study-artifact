struct Concrete(u32);

impl Concrete {
fn m(self: &std::boxed::Box<Self>) -> &u32 {
&self.0
}
}

fn main() {}