pub fn main() {
let f = f64::from_bits(0x19873cc2) as f32;
println!("{:x}", f.to_bits());
println!("{}", f == 0.);
}