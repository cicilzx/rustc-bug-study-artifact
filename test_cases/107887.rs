struct W<T: ?Sized>(*const T);

trait Trait<T: ?Sized> {
type Assoc;
}

// This would trigger the check for overlap between automatic and custom impl.
// They actually don't overlap so an impl like this should remain possible
// forever.
//
// impl Trait<u64> for dyn Trait<u32> {}
trait Indirect {}
impl Indirect for dyn Trait<u32, Assoc = ()> {}
impl<T: Indirect + ?Sized> Trait<u64> for T {
type Assoc = ();
}

trait Overlap<U: ?Sized> {
type Assoc: Default;
}
impl<T: ?Sized + EvaluateHack<W<U>>, U: ?Sized> Overlap<U> for T {
type Assoc = Box<u32>;
}
impl<U: ?Sized> Overlap<U> for dyn Trait<u32, Assoc = ()> {
type Assoc = usize;
}

// Incomplete impl where `dyn Trait<u32>: Trait<_>` does not hold, but
// `dyn Trait<u32>: Trait<u64>` does.
trait EvaluateHack<U: ?Sized> {}
impl<T: ?Sized, U: ?Sized> EvaluateHack<W<U>> for T
where
T: Trait<U, Assoc = ()>, // incompletely constrains `_` to `u32`
U: IsU64,
T: Trait<U, Assoc = ()>, // incompletely constrains `_` to `u32`
{
}

trait IsU64 {}
impl IsU64 for u64 {}

fn overlap<T: Overlap<U> + ?Sized, U>() -> T::Assoc {
Default::default()
}

fn generic_first<T: ?Sized + EvaluateHack<W<U>>, U>() {
overlap::<T, U>();
}
fn main() {
generic_first::<dyn Trait<u32, Assoc = ()>, u64>();
}