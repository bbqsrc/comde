use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::io::prelude::*;

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor};

use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;

pub type DeflateHashMap<K, V> =
    CompressedHashMap<K, V, RandomState, DeflateCompressor, DeflateDecompressor>;

#[derive(Debug, Copy, Clone)]
pub struct DeflateDecompressor;

impl<V: Decompress> Decompressor<V> for DeflateDecompressor {
    fn new() -> Self {
        DeflateDecompressor
    }

    fn decompress(&self, data: &[u8]) -> Result<V, Box<dyn Error>> {
        let mut buffer = vec![];
        let mut decoder = DeflateDecoder::new(data);
        decoder.read_to_end(&mut buffer)?;
        V::from_bytes(buffer)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DeflateCompressor;

impl<V: Compress> Compressor<V> for DeflateCompressor {
    fn new() -> Self {
        DeflateCompressor
    }

    fn compress<D: Borrow<V>>(&self, data: D) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        encoder.write(data.borrow().as_bytes())?;
        encoder.finish().map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut map = DeflateHashMap::<String, String>::new();
        map.insert("foo".into(), "bar".into());
        assert_eq!("bar".to_string(), map.get("foo").unwrap());
        assert_ne!("bap".to_string(), map.get("foo").unwrap());
    }
}
