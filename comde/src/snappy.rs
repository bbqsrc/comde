use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::io::{prelude::*, Result};

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

    fn from_reader<R: Read>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let mut decoder = snap::Reader::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SnappyCompressor;

impl<V: Compress> Compressor<V> for SnappyCompressor {
    fn new() -> Self {
        SnappyCompressor
    }

    fn compress<W: Write>(&self, writer: W, data: V) -> Result<()> {
        let mut encoder = snap::Writer::new(writer);
        std::io::copy(&mut data.to_reader(), &mut encoder);
        Ok(())
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
