use std::io::{self, Read, Write};

struct S;

impl Read for S {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl Write for S {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

fn main() {
    let s = S;
    tungstenite::accept_hdr(s, |_, _| {
        let callback = |_, _| {
            todo!()
        };

        tungstenite::accept_hdr(s, callback);
        todo!()
    });
}