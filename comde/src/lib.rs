//! # Comde
//!
//! Comde is a framework for __com__pressing and __de__compressing.

#![feature(min_const_generics)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod com;
pub mod de;

pub mod stored;

#[cfg(all(feature = "std", feature = "brotli"))]
pub mod brotli;

#[cfg(all(feature = "std", feature = "xz"))]
pub mod xz;

#[cfg(all(feature = "std", feature = "snappy"))]
pub mod snappy;

#[cfg(all(feature = "std", feature = "deflate"))]
pub mod deflate;

#[cfg(feature = "zstandard")]
pub mod zstd;

pub use com::{ByteCount, Compress, Compressor};
pub use de::{Decompress, Decompressor};
