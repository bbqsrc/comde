use std::collections::hash_map::RandomState;
use std::io::{prelude::*, Result, Seek, SeekFrom};

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor, com::ByteCount};

pub type SnappyHashMap<K, V> =
    CompressedHashMap<K, V, RandomState, SnappyCompressor, SnappyDecompressor>;

#[derive(Debug, Copy, Clone)]
pub struct SnappyDecompressor;

impl Decompressor for SnappyDecompressor {
    fn new() -> Self {
        SnappyDecompressor
    }

    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = snap::Reader::new(source);
        std::io::copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = snap::Reader::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SnappyCompressor;

impl Compressor for SnappyCompressor {
    fn new() -> Self {
        SnappyCompressor
    }

    fn compress<W: Write + Seek, V: Compress>(&self, mut writer: W, data: V) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = snap::Writer::new(writer);
        let read = std::io::copy(&mut data.to_reader(), &mut encoder)?;
        let mut writer = encoder.into_inner().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to get writer out of encoder"))?;
        let end = writer.seek(SeekFrom::Current(0))?;
        Ok(ByteCount { read, write: end - start })
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
