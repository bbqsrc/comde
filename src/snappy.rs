use std::io::{prelude::*, Result, Seek, SeekFrom};

use crate::{com::ByteCount, Compressor, Decompress, Decompressor};

#[derive(Debug, Copy, Clone)]
pub struct SnappyDecompressor;

impl Decompressor for SnappyDecompressor {
    fn new() -> Self {
        SnappyDecompressor
    }

    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = snap::read::FrameDecoder::new(source);
        std::io::copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = snap::read::FrameDecoder::new(reader);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SnappyCompressor;

impl Compressor for SnappyCompressor {
    fn new() -> Self {
        SnappyCompressor
    }

    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = snap::write::FrameEncoder::new(writer);
        let read = std::io::copy(reader, &mut encoder)?;
        let writer = encoder.into_inner().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to get writer out of encoder",
            )
        })?;
        let end = writer.seek(SeekFrom::Current(0))?;
        Ok(ByteCount {
            read,
            write: end - start,
        })
    }
}
