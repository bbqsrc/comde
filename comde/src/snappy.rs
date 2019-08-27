use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::io::prelude::*;

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor};

pub type SnappyHashMap<K, V> =
    CompressedHashMap<K, V, RandomState, SnappyCompressor, SnappyDecompressor>;

#[derive(Debug, Copy, Clone)]
pub struct SnappyDecompressor;

impl<V: Decompress> Decompressor<V> for SnappyDecompressor {
    fn new() -> Self {
        SnappyDecompressor
    }

    fn decompress(&self, data: &[u8]) -> Result<V, Box<dyn Error>> {
        let mut buffer = vec![];
        let mut decoder = snap::Reader::new(data);
        decoder.read_to_end(&mut buffer)?;
        V::from_bytes(buffer)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SnappyCompressor;

impl<V: Compress> Compressor<V> for SnappyCompressor {
    fn new() -> Self {
        SnappyCompressor
    }

    fn compress<D: Borrow<V>>(&self, data: D) -> Result<Vec<u8>, Box<dyn Error>> {
        let output = vec![];
        let mut encoder = snap::Writer::new(output);
        encoder.write(data.borrow().as_bytes())?;
        encoder
            .into_inner()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut map = SnappyHashMap::<String, String>::new();
        map.insert("foo".into(), "bar".into());
        assert_eq!("bar".to_string(), map.get("foo").unwrap());
        assert_ne!("bap".to_string(), map.get("foo").unwrap());
    }
}
