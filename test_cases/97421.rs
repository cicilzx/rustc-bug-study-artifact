pub fn expose_addr<T>(a: &T) {
a as *const T as usize;
}

fn main() {}