use std::borrow::Borrow;
use std::error::Error;

pub trait Compressor<V>
where
    V: Compress,
{
    fn new() -> Self;
    fn compress<D: Borrow<V>>(&self, data: D) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub trait Compress {
    fn as_bytes(&self) -> &[u8];
}
