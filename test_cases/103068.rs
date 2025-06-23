#[inline(always)]
fn foo() {
bar(); // line 3
}

#[inline(never)]
fn bar() {
panic!(); // line 8
}

fn main() {
foo(); // line 12
}