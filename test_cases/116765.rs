struct Factory<'a> {
value: &'a usize,
}

impl<'a> Factory<'a> {
fn generate(&self) -> Box<dyn std::fmt::Debug + 'a> {
Box::new(Value { value: self.value })
}
}


struct Owner {
value: Box<dyn std::fmt::Debug>
}


#[derive(Debug)]
struct Value<'a> {
value: &'a usize,
}


fn build_owner<'a,'b>(factory: &'b Factory<'a>) -> Owner {
let value = factory.generate();

Owner { value }
}

fn main() {
let value = 10;

let factory = Factory { value: &value };

let _owner = build_owner(&factory);
}