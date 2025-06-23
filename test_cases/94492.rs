fn main() {
    trait T {
    fn foo(&self, x: &i32) -> Option<&i32> {
    Some(x)
    }
    }
}