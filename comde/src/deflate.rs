use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::io::{prelude::*, Result};

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

    fn from_reader<R: Read>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let mut decoder = DeflateDecoder::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DeflateCompressor;

impl<V: Compress> Compressor<V> for DeflateCompressor {
    fn new() -> Self {
        DeflateCompressor
    }

    fn compress<W: Write>(&self, writer: W, data: V) -> Result<()> {
        let mut encoder = DeflateEncoder::new(writer, Compression::default());
        std::io::copy(&mut data.to_reader(), &mut encoder)?;
        encoder.finish()?;
        Ok(())
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
