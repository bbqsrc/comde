use std::io::{Read, Result};

pub trait Decompressor {
    fn new() -> Self;
    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized;

    fn from_vec<V: Decompress>(&self, bytes: Vec<u8>) -> Result<V>
    where
        Self: Sized,
    {
        let reader = std::io::Cursor::new(bytes);
        self.from_reader(reader)
    }
}

pub trait Decompress {
    fn from_reader<R: Read>(reader: R) -> Result<Self>
    where
        Self: Sized;

    fn from_vec(&self, bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        let reader = std::io::Cursor::new(bytes);
        Self::from_reader(reader)
    }
}

impl Decompress for String {
    fn from_reader<R: Read>(mut reader: R) -> Result<Self>
    where
        Self: Sized,
    {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        Ok(string)
    }
}
