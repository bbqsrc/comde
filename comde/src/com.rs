use std::io::{Read, Result, Write};

pub trait Compressor<V>
where
    V: Compress,
{
    fn new() -> Self;
    fn compress<W: Write>(&self, writer: W, data: V) -> Result<()>;
    fn to_vec(&self, data: V) -> Result<Vec<u8>> {
        let mut writer = Vec::with_capacity(128);
        self.compress(&mut writer, data)?;
        Ok(writer)
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
