use std::io::{prelude::*, Result, Seek};
use crate::{Compress, Compressor, Decompress, Decompressor, com::ByteCount};

#[derive(Debug, Copy, Clone)]
pub struct StoredDecompressor;

impl Decompressor for StoredDecompressor {
    fn new() -> Self {
        StoredDecompressor
    }

    fn copy<R: Read, W: Write>(&self, mut source: R, mut dest: W) -> Result<u64> {
        std::io::copy(&mut source, &mut dest)
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

    fn compress<W: Write + Seek, V: Compress>(&self, mut writer: W, data: V) -> Result<ByteCount> {
        let read = std::io::copy(&mut data.to_reader(), &mut writer)?;
        Ok(ByteCount { read, write: read })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash_map::CompressedHashMap;
    use std::collections::hash_map::RandomState;

    type PassthroughHashMap<K, V> = CompressedHashMap<K, V, RandomState, StoredCompressor, StoredDecompressor>;

    #[test]
    fn basic() {
        let mut map = PassthroughHashMap::<String, String>::new();
        map.insert("foo".into(), "bar".into());
        assert_eq!("bar".to_string(), map.get("foo").unwrap());
        assert_ne!("bap".to_string(), map.get("foo").unwrap());
    }
}
