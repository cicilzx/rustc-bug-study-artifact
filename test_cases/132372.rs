#![allow(async_fn_in_trait)]
pub trait foo {}

pub trait bar {
type foo: foo;
}
pub trait baz {
async fn boom<X: bar>() -> Result<(), X::foo>;
}

fn main() {}