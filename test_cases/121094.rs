use std::{future::Future, pin::Pin};

pub async fn foo(count: u32) {
if count == 0 {
return
} else {
let fut: Pin<Box<dyn Future<Output = ()>>> = Box::pin(foo(count - 1));
fut.await;
}
}

fn main(){}