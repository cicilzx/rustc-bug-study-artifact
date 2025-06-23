fn main() {}

fn foo(src: &crate::Foo) -> Option<i32> {
todo!()
}
fn bar(src: &crate::Foo) -> impl Iterator<Item = i32> {
[0].into_iter()
.filter_map(|_| foo(src))
}

struct Foo<'a>(&'a str);