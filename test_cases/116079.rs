#![feature(if_let_guard)]

static mut A: [i32; 5] = [1, 2, 3, 4, 5];

async fn fun() {
unsafe {
match A {
_ => (),
i if let Some(1) = async { Some(1) }.await => (),
_ => (),
}
}
}

fn main() {}