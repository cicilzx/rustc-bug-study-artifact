pub struct KeyHolder<const K: u8> {}

pub trait ContainsKey<const K: u8> {}

pub trait SubsetExcept<P> {}

impl<K> ContainsKey<K> for KeyHolder<K> {}

impl<P, T: ContainsKey<0>> SubsetExcept<P> for T {}

pub fn remove_key<K, S: SubsetExcept<K>>() -> S {
loop {}
}

fn foo() {
let map: KeyHolder<0> = remove_key::<_, _>();
}

fn main() {}