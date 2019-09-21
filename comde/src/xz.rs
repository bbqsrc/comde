use std::collections::hash_map::RandomState;
use std::io::{prelude::*, Result, Seek, SeekFrom};

use xz2::{read::XzDecoder, write::XzEncoder};

use crate::hash_map::CompressedHashMap;
use crate::{com::ByteCount, Compressor, Decompress, Decompressor};

#[cfg(feature = "with-phf")]
use crate::phf::CompressedPhfMap;

pub type XzHashMap<K, V> = CompressedHashMap<K, V, RandomState, XzCompressor, XzDecompressor>;

#[cfg(feature = "with-phf")]
pub type XzPhfMap<K, V> = CompressedPhfMap<K, V, XzCompressor>;

#[derive(Debug, Copy, Clone)]
pub struct XzDecompressor;

impl Decompressor for XzDecompressor {
    fn new() -> Self {
        XzDecompressor
    }

    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = XzDecoder::new(source);
        std::io::copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = XzDecoder::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XzCompressor;

impl Compressor for XzCompressor {
    fn new() -> Self {
        XzCompressor
    }

    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = XzEncoder::new(writer, 9);
        let read = std::io::copy(reader, &mut encoder)?;
        let end = encoder.finish()?.seek(SeekFrom::Current(0))?;
        Ok(ByteCount {
            read,
            write: end - start,
        })
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
