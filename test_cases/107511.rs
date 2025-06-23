fn main() {
let mut sum = 0;
let a = [0, 10, 20, 30];
for i in 0..a.len() {
sum += a[i];
}
println!("{sum}");
}