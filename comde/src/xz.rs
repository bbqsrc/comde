use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::io::{prelude::*, BufWriter, Result};

use xz2::{read::XzDecoder, write::XzEncoder};

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor};

#[cfg(feature = "with-phf")]
use crate::phf::CompressedPhfMap;

pub type XzHashMap<K, V> = CompressedHashMap<K, V, RandomState, XzCompressor, XzDecompressor>;

#[cfg(feature = "with-phf")]
pub type XzPhfMap<K, V> = CompressedPhfMap<K, V, XzCompressor>;

#[derive(Debug, Copy, Clone)]
pub struct XzDecompressor;

impl<V: Decompress> Decompressor<V> for XzDecompressor {
    fn new() -> Self {
        XzDecompressor
    }

    fn from_reader<R: Read>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let mut decoder = XzDecoder::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XzCompressor;

impl<V: Compress> Compressor<V> for XzCompressor {
    fn new() -> Self {
        XzCompressor
    }

    fn compress<W: Write>(&self, writer: W, data: V) -> Result<()> {
        let mut encoder = XzEncoder::new(writer, 9);
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
        let mut map = XzHashMap::<String, String>::new();
        map.insert("foo".into(), "bar".into());
        assert_eq!("bar".to_string(), map.get("foo").unwrap());
        assert_ne!("bap".to_string(), map.get("foo").unwrap());
    }

    #[test]
    #[cfg(feature = "with-phf")]
    fn with_phf() {}
}
