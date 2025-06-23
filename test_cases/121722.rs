#[repr(C)]
struct Foo {
_: u8,
}

#[repr(C)]
struct D {
_: Foo,
}

fn main(){}