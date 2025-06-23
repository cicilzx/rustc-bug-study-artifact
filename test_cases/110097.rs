pub fn change_value(val: Result<i32, ()>) -> Result<i32, ()> {
if let Ok(x) = val {
Ok(x * 2)
} else {
Err(())
}
}

pub fn change_value2(val: Result<i32, ()>) -> Result<i32, ()> {
match val {
Ok(x) => Ok(x * 2),
Err(()) => Err(())
}
}

fn main() {}