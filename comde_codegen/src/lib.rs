use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

use byte_string::ByteString;
use phf_shared::PhfHash;

use comde::{Compress, Compressor};

#[cfg(feature = "xz")]
pub type XzMap<K, V> = Map<K, V, comde::xz::XzCompressor>;

#[cfg(feature = "xz")]
#[macro_export]
macro_rules! xz_map {
    ($ty:ty) => {
        XzMap::new("::comde::xz::XzDecompressor", stringify!($ty))
    };
}

#[cfg(feature = "deflate")]
pub type DeflateMap<K, V> = Map<K, V, comde::deflate::DeflateCompressor>;

#[cfg(feature = "deflate")]
#[macro_export]
macro_rules! deflate_map {
    ($ty:ty) => {
        SnappyMap::new("::comde::deflate::DeflateDecompressor", stringify!($ty))
    };
}

#[cfg(feature = "snappy")]
pub type SnappyMap<K, V> = Map<K, V, comde::snappy::SnappyCompressor>;

#[cfg(feature = "snappy")]
#[macro_export]
macro_rules! snappy_map {
    ($ty:ty) => {
        SnappyMap::new("::comde::snappy::SnappyDecompressor", stringify!($ty))
    };
}

#[cfg(feature = "zstandard")]
pub type ZstdMap<K, V> = Map<K, V, comde::zstd::ZstdCompressor>;

#[cfg(feature = "zstandard")]
#[macro_export]
macro_rules! zstd_map {
    ($ty:ty) => {
        ZstdMap::new("::comde::zstd::ZstdDecompressor", stringify!($ty))
    };
}

pub struct Map<K, V, C>
where
    K: Hash + PhfHash + Eq + Debug,
    V: Compress,
    C: Compressor,
{
    map: phf_codegen::Map<K>,
    compressor: C,
    decompressor_type: &'static str,
    value_type: &'static str,
    __value: std::marker::PhantomData<V>,
}

impl<K, V, C> Map<K, V, C>
where
    K: Hash + PhfHash + Eq + Debug,
    V: Compress,
    C: Compressor,
{
    pub fn new(decompressor_type: &'static str, value_type: &'static str) -> Map<K, V, C> {
        Map {
            map: phf_codegen::Map::new(),
            compressor: C::new(),
            decompressor_type,
            value_type,
            __value: std::marker::PhantomData::<V>,
        }
    }

    #[inline]
    pub fn entry(&mut self, key: K, value: V) -> &mut Map<K, V, C> {
        let bytes = self.compressor.to_vec(value).unwrap();
        self.map
            .entry(key, &format!("{:?}", ByteString::new(bytes)));
        self
    }

    #[inline]
    pub fn phf_path(&mut self, path: &str) -> &mut Map<K, V, C> {
        self.map.phf_path(path);
        self
    }

    #[inline]
    pub fn build<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        w.write(b"::comde::phf::CompressedPhfMap {\n    map: ")?;
        self.map.build(w)?;
        w.write(b",\n    decompressor_ty: std::marker::PhantomData::<")?;
        w.write(self.decompressor_type.as_bytes())?;
        w.write(b">,\n    value_ty: std::marker::PhantomData::<")?;
        w.write(self.value_type.as_bytes())?;
        w.write(b">\n}\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    #[cfg(feature = "snappy")]
    fn basic_snappy() {
        let mut map = snappy_map!(String);

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("snappy: {}", String::from_utf8(out.into_inner()).unwrap());
    }

    #[test]
    #[cfg(feature = "xz")]
    fn basic_xz() {
        let mut map = xz_map!(String);

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("xz: {}", String::from_utf8(out.into_inner()).unwrap());
    }

    #[test]
    #[cfg(feature = "deflate")]
    fn basic_deflate() {
        let mut map = deflate_map!(String);

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("deflate: {}", String::from_utf8(out.into_inner()).unwrap());
    }

    #[test]
    #[cfg(feature = "zstandard")]
    fn basic_zstd() {
        let mut map = zstd_map!(String);

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("zstd: {}", String::from_utf8(out.into_inner()).unwrap());
    }
}
