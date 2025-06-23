use std::fmt;

pub trait ConnectionDriver {
fn name(&self) -> String;
}
impl fmt::Debug for dyn ConnectionDriver {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
f.write_fmt(format_args!("name: {}", self.name()))
}
}

struct Wifi;
impl ConnectionDriver for Wifi {
fn name(&self) -> String {
String::from("wifi")
}
}

// type C = Box<dyn ConnectionDriver>; // this can work
type C = Box<dyn ConnectionDriver + Send>; // this can't work
pub struct Connection(pub C);


fn main() {
let conn = Connection(Box::new(Wifi));
println!("{:?}", conn.0);
}