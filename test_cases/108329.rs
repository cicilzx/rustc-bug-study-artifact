pub fn take (&mut self) -> Vec<T> {
let mut buf = Vec::with_capacity(self.buffer.len());
buf.copy_from_slice(self.buffer[..]);
self.index = S - 1;
buf
}

fn main() {}