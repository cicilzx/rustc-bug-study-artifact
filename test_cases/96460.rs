use core::marker::PhantomData;

fn main() {
    fn weird() -> PhantomData<impl Sized> {
    PhantomData
    }
}