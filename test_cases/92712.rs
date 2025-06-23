#![allow(unused)]
#![feature(const_fn_trait_bound, const_trait_impl)]

fn main() {
    const fn f<T: ~const Drop>(x: T) {}

    struct UnconstDrop;

    impl Drop for UnconstDrop {
    fn drop(&mut self) {}
    }

    const X: () = f(UnconstDrop);
}