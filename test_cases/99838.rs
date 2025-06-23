struct U16(u16);

impl Drop for U16 {
fn drop(&mut self) {
println!("{:p}", self);
}
}

struct HasDrop;

impl Drop for HasDrop {
fn drop(&mut self) {}
}

#[repr(packed)]
struct Misalign(u8, Wrapper);

struct Wrapper {
_a: U16,
b: HasDrop,
}

fn main() {
let m = Misalign(
0,
Wrapper {
_a: U16(10),
b: HasDrop,
},
);
let _x = m.1.b;
}