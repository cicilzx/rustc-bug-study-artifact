use std::sync::Mutex;

static MUTEX: Mutex<Option<&'static str>> = Mutex::new(None);

trait Foo {
fn foo<'a: 'static>(&self) -> impl Sized;
}

impl Foo for str {
fn foo<'a: 'static>(&'a self) -> impl Sized + 'a {
*MUTEX.lock().unwrap() = Some(self);
}
}

fn call_foo<T: Foo + ?Sized>(s: &T) {
s.foo();
}

fn main() {
let s = String::from("hello, world");
call_foo(s.as_str());
drop(s);
println!("> {}", MUTEX.lock().unwrap().unwrap());
}