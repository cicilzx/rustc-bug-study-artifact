pub fn foo<const BAR: bool> () {}

pub fn main() {
// warning: unnecessary braces around const expression
foo::<{cfg!(feature = "foo")}>();
}