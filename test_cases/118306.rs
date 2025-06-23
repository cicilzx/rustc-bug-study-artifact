pub fn faster(input: u64) -> u64 {
match input % 4 {
0 => 0,
1 | 2 => 1,
3 => 2,
_ => unreachable!(),
}
}

pub fn branchy(input: u64) -> u64 {
match input % 4 {
1 | 2 => 1,
3 => 2,
_ => 0,
}
}

fn main() {}