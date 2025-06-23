pub fn myfunc() -> i32 {
let mut c :i32 = 1;
c = 1 ;
if !c != 0 {
return 1;
}
panic!("Reached end of non-void function");
}

pub fn main() {
let e = myfunc();
println!("e={}", e);
}