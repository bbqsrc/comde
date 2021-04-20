//! # Comde
//!
//! Comde is a framework for __com__pressing and __de__compressing.

// #![cfg_attr(not(feature = "std"), no_std)]

pub mod com;
pub mod de;

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

pub use com::{ByteCount, Compress, Compressor};
pub use de::{Decompress, Decompressor};
