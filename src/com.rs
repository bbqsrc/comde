//! Generic data structure compression framework.

use bare_io::{Read, Result, Seek, Write};

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

    #[cfg(feature = "std")]
    fn to_vec<V: Compress>(&self, data: V) -> Result<Vec<u8>> {
        let mut writer = bare_io::Cursor::new(Vec::with_capacity(128));
        self.compress(&mut writer, &mut data.to_reader())?;
        Ok(writer.into_inner())
    }
}

pub trait Compress {
    type Reader: Read;
    fn to_reader(self) -> Self::Reader;
}

#[cfg(feature = "std")]
impl Compress for String {
    type Reader = bare_io::Cursor<String>;

    fn to_reader(self) -> Self::Reader {
        bare_io::Cursor::new(self)
    }
}

#[cfg(feature = "std")]
impl<'a> Compress for &'a str {
    type Reader = bare_io::Cursor<&'a str>;

    fn to_reader(self) -> Self::Reader {
        bare_io::Cursor::new(self)
    }
}

#[cfg(feature = "std")]
impl<'a> Compress for &'a Vec<u8> {
    type Reader = bare_io::Cursor<Self>;

    fn to_reader(self) -> Self::Reader {
        bare_io::Cursor::new(self)
    }
}
