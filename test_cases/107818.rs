#[derive(PartialEq, Eq, Clone)]
struct Foo {
field: i64
}

const FOO: Foo = Foo {
field: 5,
};

fn main() {
match FOO {
FOO => unreachable!(),
_ => unreachable!(),
}
}