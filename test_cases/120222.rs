#![feature(trait_upcasting)]

trait Trait<T>: Super {}
trait Super {}

impl<S, T> Trait<T> for S {}
impl<S> Super for S {}

fn main() {
let p: *const dyn Trait<u8> = &();
let p = p as *const dyn Trait<u16>;
let _p = p as *const dyn Super; // this is where miri complains already
}