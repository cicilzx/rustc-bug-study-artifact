use core::cmp::Ordering;

pub fn test1(call: fn() -> Ordering) -> i32 {
match call() {
Ordering::Less => -1,
Ordering::Equal => 0,
Ordering::Greater => 1,
}
}

pub fn test2(call: fn() -> Ordering) -> i32 {
call() as i32
}

fn main() {}