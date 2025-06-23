#![feature(type_alias_impl_trait)]
fn main() {
    type Ty<'a> = impl FnOnce() -> &'a str;
    fn defining(s: &str) -> Ty<'_> { move || s }
    fn execute(ty: Ty<'_>) -> &str { ty() }
}