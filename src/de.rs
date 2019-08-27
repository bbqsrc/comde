use std::error::Error;

pub trait Decompressor<V>
where
    V: Decompress,
{
    fn new() -> Self;
    fn decompress(&self, data: &[u8]) -> Result<V, Box<dyn Error>>;
}

pub trait Decompress {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}
