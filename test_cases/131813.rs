#![feature(trait_upcasting)]

trait Pollable {
#[allow(unused)]
fn poll(&self) {}
}
trait FileIo: Pollable + Send {
fn read(&self) {}
}
trait Terminal: Send + FileIo {}

struct A;

impl Pollable for A {}
impl FileIo for A {}
impl Terminal for A {}

fn main() {
let a = A;

let b = &a as &dyn Terminal;
let c = b as &dyn FileIo;

c.read();
}