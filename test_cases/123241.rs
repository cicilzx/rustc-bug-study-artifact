#![feature(async_closure)]

fn outlives<'a, T: 'a>(_: T) {}

fn hello<'a>(x: &'a i32) {
let c = async || {
outlives::<'a>(async {
let y = *x;
});
};
}

fn main() {}