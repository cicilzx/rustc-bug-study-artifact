unsafe fn src(x: &&u8) -> bool {
let y = **x;
unknown();
**x == y
}

static mut SUSSY: *mut u8 = core::ptr::null_mut();

#[inline(never)]
unsafe fn unknown() {
*SUSSY = 1;
}

fn main() {
let mut s = 0;
unsafe {
SUSSY = core::ptr::addr_of_mut!(s);
println!("{}", src(&*core::ptr::addr_of!(SUSSY).cast::<&u8>()));
}
}