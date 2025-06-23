use std::any::TypeId;
use std::any::Any;
use std::hint::black_box;
struct A<T:?Sized+'static> {
a: i32,
b: T
}

impl<T:?Sized+'static> A<T> {
fn bb(&self) -> TypeId {
self.b.type_id()
}
}

pub fn main() {
let mut a0 = A{a: 8, b: 9 as i32};
let mut a: &mut A<dyn Any> = &mut a0;
println!("{:?}",a.bb());
println!("{:?}",a.b.type_id());
println!("{:?}",std::any::TypeId::of::<i32>());
}