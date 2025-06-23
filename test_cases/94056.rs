fn main() {
    fn testcase(items: &[u64]) -> Abc {
    let mut abc = Abc { bar: Vec::new() };
    items.iter().for_each(|item| abc.update(*item));
    abc
    }

    struct Abc {
    bar: Vec<u64>,
    }

    impl Abc {
    fn update(mut self, bar: u64) {
    // ^^^^^^^^ the issue is here, this should be `&mut self`
    self.bar.push(bar);
    }
    }
}