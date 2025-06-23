pub trait LendingIterator {
type Item<'a>
where
Self: 'a;
fn for_each(self, _: Box<dyn FnMut(Self::Item<'_>)>)
where
Self: Sized,
{
}
}
pub struct Query<'q>(&'q ());

impl<'q> LendingIterator for Query<'q> {
type Item<'a> = &'a () where Self: 'a;
}

fn main() {
let q = Query(&());
LendingIterator::for_each(q, Box::new(|_| {}));
}