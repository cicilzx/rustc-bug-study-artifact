enum State {
A([u8; 753]),
B([u8; 753]),
//C,
}

unsafe fn update(s: *mut State) {
let S::A(v) = s.read() else { unreachable_unchecked() };
s.write(S::B(v));
}

fn main() {}