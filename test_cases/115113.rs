pub struct Endian;

#[allow(dead_code)]
pub struct EndianSlice<'input> {
slice: &'input [u8],
endian: Endian,
}

pub fn test(s: &[u8]) {
let slice = EndianSlice { slice: s, endian: Endian };
}

fn main(){}