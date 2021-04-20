use crate::{com::ByteCount, Compressor, Decompress, Decompressor};
use std::io::copy;
use std::io::{Read, Result, Seek, Write};

#[derive(Debug, Copy, Clone)]
pub struct StoredDecompressor;

impl Decompressor for StoredDecompressor {
    fn new() -> Self {
        StoredDecompressor
    }

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
