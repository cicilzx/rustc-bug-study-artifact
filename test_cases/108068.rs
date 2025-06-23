fn main() {
    type T = [u8; 256];
    pub fn f(a: T, b: fn(_: T, _: T)) {
    b(a, a)
    }
}