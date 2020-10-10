use std::io::{prelude::*, Result, Seek, SeekFrom};

use crate::{com::ByteCount, Compressor, Decompress, Decompressor};
#[derive(Debug, Copy, Clone)]
pub struct BrotliDecompressor;

impl Decompressor for BrotliDecompressor {
    fn new() -> Self {
        BrotliDecompressor
    }

    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = brotli::Decompressor::new(source, 4096);
        std::io::copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = brotli::Decompressor::new(reader, 4096);
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BrotliCompressor;

impl Compressor for BrotliCompressor {
    fn new() -> Self {
        BrotliCompressor
    }

    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = brotli::enc::writer::CompressorWriter::new(writer, 4096, 11, 22);
        let read = std::io::copy(reader, &mut encoder)?;
        let writer = encoder.into_inner();
        let end = writer.seek(SeekFrom::Current(0))?;
        Ok(ByteCount {
            read,
            write: end - start,
        })
    }
}
