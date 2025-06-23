async fn clone_async_block(value: String) {
for _ in 0..10 {
drop(async { drop(value) })
}
}

fn main(){}