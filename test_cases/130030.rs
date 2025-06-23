fn main() {
let ptr: *const dyn Fn() = &|| ();
_ = ptr as *const dyn Fn(u8);
}