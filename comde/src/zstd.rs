use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::io::prelude::*;

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

    fn decompress(&self, data: &[u8]) -> Result<V, Box<dyn Error>> {
        let mut buffer = vec![];
        let mut decoder = Decoder::new(data)?;
        decoder.read_to_end(&mut buffer)?;
        V::from_bytes(buffer)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ZstdCompressor;

impl<V: Compress> Compressor<V> for ZstdCompressor {
    fn new() -> Self {
        ZstdCompressor
    }

    fn compress<D: Borrow<V>>(&self, data: D) -> Result<Vec<u8>, Box<dyn Error>> {
        let output = vec![];
        let mut encoder = Encoder::new(output, 21)?;
        encoder.write(data.borrow().as_bytes())?;
        encoder.finish().map_err(|e| Box::new(e) as Box<dyn Error>)
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
