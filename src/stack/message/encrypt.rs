extern crate lz4;

use lz4::{Decoder, EncoderBuilder};

fn main() {
    println!("LZ4 version: {}", lz4::version());

    for path in env::args().skip(1).map(PathBuf::from) {
        if let Some("lz4") = path.extension().and_then(|e| e.to_str()) {
            decompress(&path, &path.with_extension("")).unwrap();
        } else {
            compress(&path, &path.with_extension("lz4")).unwrap();
        }
    }
}

#[cfg(test)]
#[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }