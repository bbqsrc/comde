//! # Comde
//! Comde is a framework for _com_pressing and _de_compressing.

pub mod com;
pub mod de;
pub mod hash_map;

pub mod stored;

#[cfg(feature = "brotli")]
pub mod brotli;

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

pub use com::{ByteCount, Compress, Compressor};
pub use de::{Decompress, Decompressor};
pub use hash_map::CompressedHashMap;
