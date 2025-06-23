struct A<'a> {
dat: &'a (),
}

impl<'a> A<'a> {
async fn a(&self) -> impl Iterator<Item = std::iter::Repeat<()>> {
std::iter::repeat(()).map(|()| std::iter::repeat(()))
}
}

fn main() {}