use std::io::{Read, Result, Seek, SeekFrom, Write};

#[cfg(feature = "nightly")]
use std::io::copy;

#[cfg(not(feature = "nightly"))]
use std::io::copy;

use xz2::{read::XzDecoder, write::XzEncoder};

use crate::{com::ByteCount, Compressor, Decompress, Decompressor};

#[derive(Debug, Copy, Clone)]
pub struct XzDecompressor;

impl Decompressor for XzDecompressor {
    fn new() -> Self {
        XzDecompressor
    }

    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = XzDecoder::new(source);
        copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = XzDecoder::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XzCompressor;

impl Compressor for XzCompressor {
    fn new() -> Self {
        XzCompressor
    }

    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = XzEncoder::new(writer, 9);
        let read = copy(reader, &mut encoder)?;
        let end = encoder.finish()?.seek(SeekFrom::Current(0))?;
        Ok(ByteCount {
            read,
            write: end - start,
        })
    }
}
