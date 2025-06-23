trait MyTrait {
type T;

fn bar(self) -> Self::T;
}

fn foo<A: MyTrait, B>(a: A) -> B {
return a.bar();
}

fn main() {}