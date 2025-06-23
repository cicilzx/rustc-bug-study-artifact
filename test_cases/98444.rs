fn main() {
let xs: [i32; 5] = [1, 2, 3, 4, 5];
let _ = &xs; // this line suppresses the `unconditional_panic` lint
let _ = xs[7];
}