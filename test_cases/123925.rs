fn main() {
    fn bar(_: impl IntoIterator<Item = ()>) {}
    bar(Some(()).iter().copied().collect());
}