pub mod com;
pub mod de;
pub mod hash_map;

#[cfg(feature = "xz")]
pub mod xz;

#[cfg(feature = "snappy")]
pub mod snappy;

#[cfg(feature = "deflate")]
pub mod deflate;

#[cfg(feature = "zstandard")]
pub mod zstd;

#[cfg(feature = "with-phf")]
pub mod phf;

pub use com::{Compress, Compressor};
pub use de::{Decompress, Decompressor};
pub use hash_map::CompressedHashMap;
