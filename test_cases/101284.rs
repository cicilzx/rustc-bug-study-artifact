#![warn(rust_2021_compatibility)]
use std::sync::Arc;

pub struct Warns {
_test: Arc<String>, // Removing this Arc results in no warning
foo: String,
}

impl Warns {
pub fn test(self) -> std::io::Result<()> {
let closure = move || {
let _ = self.foo;
Ok(())
};
closure()
}
}

fn main() {}