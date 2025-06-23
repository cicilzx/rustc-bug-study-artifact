struct E {
g: i32,
f: i32,
}

fn test(a: &E, b: &E) -> bool {
a.g == b.g && a.f == b.f
}

fn main() {}