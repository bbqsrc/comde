use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::io::{prelude::*, BufWriter};

use xz2::{read::XzDecoder, write::XzEncoder};

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor};

#[cfg(feature = "with-phf")]
use crate::phf::CompressedPhfMap;

pub type XzHashMap<K, V> = CompressedHashMap<K, V, RandomState, XzCompressor, XzDecompressor>;
#[cfg(feature = "with-phf")]
pub type XzPhfMap<K, V> = CompressedPhfMap<K, V, XzCompressor, XzDecompressor>;

#[derive(Debug, Copy, Clone)]
pub struct XzDecompressor;

impl<V: Decompress> Decompressor<V> for XzDecompressor {
    fn new() -> Self {
        XzDecompressor
    }

    fn decompress(&self, data: &[u8]) -> Result<V, Box<dyn Error>> {
        let mut buffer = vec![];
        let mut decoder = XzDecoder::new(data);
        decoder.read_to_end(&mut buffer)?;
        V::from_bytes(buffer)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XzCompressor;

impl<V: Compress> Compressor<V> for XzCompressor {
    fn new() -> Self {
        XzCompressor
    }

    fn compress<D: Borrow<V>>(&self, data: D) -> Result<Vec<u8>, Box<dyn Error>> {
        let output = BufWriter::new(vec![]);
        let mut encoder = XzEncoder::new(output, 9);
        encoder.write(data.borrow().as_bytes())?;
        encoder
            .finish()
            .map(|x| x.into_inner().unwrap())
            .map_err(|e| Box::new(e) as Box<dyn Error>)
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
    fn with_phf() {
        
    }
}
