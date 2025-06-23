#![allow(warnings)]

fn main() {
let mut data = vec![1, 2, 3];
let mut i = indices(&data);
data.push(4);
i.next();
}

fn indices<T>(
slice: &[T],
) -> impl Iterator<Item = usize> {
0 .. slice.len()
}