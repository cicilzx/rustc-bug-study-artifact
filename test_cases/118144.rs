struct V(&mut i32);

fn nested(v: &V) {
|| {
V(_somename) = v;
v.0 = 0;
};
}

fn main() {}