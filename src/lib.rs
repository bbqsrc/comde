pub mod com;
pub mod de;
pub mod hash_map;

#[cfg(feature = "xz")]
pub mod xz2;

#[cfg(feature = "snappy")]
pub mod snappy;

#[cfg(feature = "deflate")]
pub mod deflate;

pub use com::{Compress, Compressor};
pub use de::{Decompress, Decompressor};
pub use hash_map::CompressedHashMap;

use std::error::Error;

impl Compress for String {
    fn as_bytes(&self) -> &[u8] {
        String::as_bytes(&self)
    }
}

impl Decompress for String {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        String::from_utf8(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
