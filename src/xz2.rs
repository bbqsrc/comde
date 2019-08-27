use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::io::{prelude::*, BufWriter};

use xz2::{read::XzDecoder, write::XzEncoder};

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor};

pub type Xz2HashMap<K, V> = CompressedHashMap<K, V, RandomState, Xz2Compressor, Xz2Decompressor>;

#[derive(Debug, Copy, Clone)]
pub struct Xz2Decompressor;

impl<V: Decompress> Decompressor<V> for Xz2Decompressor {
    fn new() -> Self {
        Xz2Decompressor
    }

    fn decompress(&self, data: &[u8]) -> Result<V, Box<dyn Error>> {
        let mut buffer = vec![];
        let mut decoder = XzDecoder::new(data);
        decoder.read_to_end(&mut buffer)?;
        V::from_bytes(buffer)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Xz2Compressor;

impl<V: Compress> Compressor<V> for Xz2Compressor {
    fn new() -> Self {
        Xz2Compressor
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
        let mut map = Xz2HashMap::<String, String>::new();
        map.insert("foo".into(), "bar".into());
        assert_eq!("bar".to_string(), map.get("foo").unwrap());
        assert_ne!("bap".to_string(), map.get("foo").unwrap());
    }
}
