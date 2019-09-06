use std::collections::hash_map::RandomState;
use std::io::{prelude::*, Result, Seek, SeekFrom};

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor, com::ByteCount};

use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;

pub type DeflateHashMap<K, V> =
    CompressedHashMap<K, V, RandomState, DeflateCompressor, DeflateDecompressor>;

#[derive(Debug, Copy, Clone)]
pub struct DeflateDecompressor;

impl Decompressor for DeflateDecompressor {
    fn new() -> Self {
        DeflateDecompressor
    }

    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = DeflateDecoder::new(source);
        std::io::copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = DeflateDecoder::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DeflateCompressor;

impl Compressor for DeflateCompressor {
    fn new() -> Self {
        DeflateCompressor
    }

    fn compress<W: Write + Seek, V: Compress>(&self, mut writer: W, data: V) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = DeflateEncoder::new(writer, Compression::default());
        let read = std::io::copy(&mut data.to_reader(), &mut encoder)?;
        let end = encoder.finish()?.seek(SeekFrom::Current(0))?;
        Ok(ByteCount { read, write: end - start })
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
