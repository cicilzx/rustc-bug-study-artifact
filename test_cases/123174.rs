pub fn closed(v: &Vec<u8>) -> &[u8] {
v
}

pub fn open(v: &Vec<u8>) -> &[u8] {
vec_deref(v)
}

#[inline]
fn vec_deref<T>(v: &Vec<T>) -> &[T] {
unsafe { std::slice::from_raw_parts(v.as_ptr(), v.len()) }
}

fn main() {}