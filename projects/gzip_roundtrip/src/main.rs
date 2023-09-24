//! See [https://github.com/rust-lang/flate2-rs/blob/main/fuzz/fuzz_targets/fuzz_gz_roundtrip.rs]

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::Read;

fn main() {
    let data = b"Hi there GZIP";
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    let result = encoder.finish().unwrap();
    let mut r = GzDecoder::new(&result[..]);
    let mut ret = Vec::new();
    r.read_to_end(&mut ret).unwrap();
    assert!(ret == data);
}
