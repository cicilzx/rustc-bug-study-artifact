use std::task::Poll;

pub fn poll(val: Poll<Result<Option<Vec<u8>>, u8>>) {
match val {
Poll::Ready(Ok(Some(_trailers))) => {}
Poll::Ready(Err(_err)) => {}
Poll::Ready(Ok(None)) => {}
Poll::Pending => {}
}
}

fn main() {}