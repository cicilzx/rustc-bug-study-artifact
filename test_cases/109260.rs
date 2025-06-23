fn main() {
let mut arr_0: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

let ptr_0 = Box::new(arr_0);

let ptr_v = ptr_0[11];
let arr_v = arr_0[11];
println!("v:{}", ptr_v);
}