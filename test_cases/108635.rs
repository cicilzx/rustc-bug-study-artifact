trait Trait {
type Item<'a>: 'a;
}

fn assert_static<T: 'static>(_: T) {}

fn test_args<I: Trait>() {
let closure = |a, _b| assert_static(a);
//~^ ERROR the associated type may not live long enough

closure(None::<I::Item::<'_>>, &None::<I::Item::<'_>>);
}

fn test_upvars<I: Trait>() {
let upvars = (None::<I::Item::<'_>>, &None::<I::Item::<'_>>);
let _closure = || {
let (a, _b) = upvars;
assert_static(a);
//~^ ERROR the associated type may not live long enough
};
}

fn main() {}