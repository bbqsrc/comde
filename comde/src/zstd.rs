use std::collections::hash_map::RandomState;
use std::io::{Result, Seek, SeekFrom, prelude::*};

use zstd::stream::read::Decoder;
use zstd::stream::write::Encoder;

use crate::hash_map::CompressedHashMap;
use crate::{Compress, Compressor, Decompress, Decompressor, com::ByteCount};

pub type ZstdHashMap<K, V> = CompressedHashMap<K, V, RandomState, ZstdCompressor, ZstdDecompressor>;

#[derive(Debug, Copy, Clone)]
pub struct ZstdDecompressor;

impl Decompressor for ZstdDecompressor {
    fn new() -> Self {
        ZstdDecompressor
    }

    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = Decoder::new(source)?;
        std::io::copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = Decoder::new(reader)?;
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ZstdCompressor;

impl Compressor for ZstdCompressor {
    fn new() -> Self {
        ZstdCompressor
    }

    fn compress<W: Write + Seek, V: Compress>(&self, mut writer: W, data: V) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = Encoder::new(writer, 21)?;
        let read = std::io::copy(&mut data.to_reader(), &mut encoder)?;
        let mut writer = encoder.finish()?;
        let end = writer.seek(SeekFrom::Current(0))?;
        let write = end - start;
        Ok(ByteCount { read, write })
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
