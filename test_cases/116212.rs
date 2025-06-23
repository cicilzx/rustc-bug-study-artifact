#![feature(never_type)]

use std::mem::MaybeUninit;

struct Foo {
x: u8,
y: !,
}

fn test_panic_msg<T, F: (FnOnce() -> T) + 'static>(op: F) {
op();
}

fn main() {
unsafe {
test_panic_msg(|| MaybeUninit::<Foo>::uninit().assume_init());
}
}