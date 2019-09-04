use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::io::prelude::*;
use std::io::Result;

use zstd::stream::read::Decoder;
use zstd::stream::write::Encoder;

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor};

pub type ZstdHashMap<K, V> = CompressedHashMap<K, V, RandomState, ZstdCompressor, ZstdDecompressor>;

#[derive(Debug, Copy, Clone)]
pub struct ZstdDecompressor;

impl<V: Decompress> Decompressor<V> for ZstdDecompressor {
    fn new() -> Self {
        ZstdDecompressor
    }

    fn from_reader<R: Read>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let mut decoder = Decoder::new(reader)?;
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ZstdCompressor;

impl<V: Compress> Compressor<V> for ZstdCompressor {
    fn new() -> Self {
        ZstdCompressor
    }

    fn compress<W: Write>(&self, writer: W, data: V) -> Result<()> {
        let mut encoder = Encoder::new(writer, 21)?;
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
        let mut map = ZstdHashMap::<String, String>::new();
        map.insert("foo".into(), "bar".into());
        assert_eq!("bar".to_string(), map.get("foo").unwrap());
        assert_ne!("bap".to_string(), map.get("foo").unwrap());
    }
}
