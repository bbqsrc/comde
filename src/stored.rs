use crate::{com::ByteCount, Compressor, Decompress, Decompressor};
use bare_io::{Read, Write, Result, Seek};

#[cfg(feature = "nightly")]
use bare_io::copy;

#[cfg(not(feature = "nightly"))]
use std::io::copy;

#[cfg(feature = "nightly")]
const BUF_SIZE: usize = 8 * 1024;

#[derive(Debug, Copy, Clone)]
pub struct StoredDecompressor;

impl Decompressor for StoredDecompressor {
    fn new() -> Self {
        StoredDecompressor
    }

    #[cfg(feature = "nightly")]
    fn copy<R: Read, W: Write>(&self, mut source: R, mut dest: W) -> Result<u64> {
        copy::<_, _, BUF_SIZE>(&mut source, &mut dest)
    }

    #[cfg(not(feature = "nightly"))]
    fn copy<R: Read, W: Write>(&self, mut source: R, mut dest: W) -> Result<u64> {
        copy(&mut source, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        V::from_reader(reader)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StoredCompressor;

impl Compressor for StoredCompressor {
    fn new() -> Self {
        StoredCompressor
    }

    #[cfg(feature = "nightly")]
    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let read = copy::<_, _, BUF_SIZE>(reader, writer)?;
        Ok(ByteCount { read, write: read })
    }

    #[cfg(not(feature = "nightly"))]
    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let read = copy(reader, writer)?;
        Ok(ByteCount { read, write: read })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::hash_map::CompressedHashMap;
//     use std::collections::hash_map::RandomState;

//     type PassthroughHashMap<K, V> =
//         CompressedHashMap<K, V, RandomState, StoredCompressor, StoredDecompressor>;

//     #[test]
//     fn basic() {
//         let mut map = PassthroughHashMap::<String, String>::new();
//         map.insert("foo".into(), "bar".into());
//         assert_eq!("bar".to_string(), map.get("foo").unwrap());
//         assert_ne!("bap".to_string(), map.get("foo").unwrap());
//     }
// }
