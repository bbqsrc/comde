//! Generic data structure compression framework.

use std::io::{Read, Result, Seek, Write};

pub struct ByteCount {
    /// Bytes read from the reader, before being compressed.
    pub read: u64,

    /// Bytes written to the writer, after being compressed.
    pub write: u64,
}

pub trait Compressor {
    fn new() -> Self;
    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount>;
    fn to_vec<V: Compress>(&self, data: V) -> Result<Vec<u8>> {
        let mut writer = std::io::Cursor::new(Vec::with_capacity(128));
        self.compress(&mut writer, &mut data.to_reader())?;
        Ok(writer.into_inner())
    }
}

pub trait Compress {
    type Reader: Read;
    fn to_reader(self) -> Self::Reader;
}

impl Compress for String {
    type Reader = std::io::Cursor<String>;

    fn to_reader(self) -> Self::Reader {
        std::io::Cursor::new(self)
    }
}

impl<'a> Compress for &'a str {
    type Reader = std::io::Cursor<&'a str>;

    fn to_reader(self) -> Self::Reader {
        std::io::Cursor::new(self)
    }
}

impl<'a> Compress for &'a Vec<u8> {
    type Reader = std::io::Cursor<Self>;

    fn to_reader(self) -> Self::Reader {
        std::io::Cursor::new(self)
    }
}
