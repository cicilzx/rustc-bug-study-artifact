fn main() {
for x in 0..1 {
let mut value = 0u8;
value = x.try_into().unwrap_or(42);
assert_eq!(value, 0);
}
}