#![feature(let_chains)]

struct F(Box<()>);

impl F {
fn s(&self) -> Option<&str> {
None
}
}

fn cex() -> Option<F> {
None
}

pub fn main() {
if false
&& let Some(ce) = cex()
&& let Some(_ce) = ce.s()
{
}
}