use std::hash::Hash;

use byte_string::ByteString;
use phf_shared::{PhfHash, FmtConst};
use delegate::delegate;

use comde::{Compress, Compressor};

#[cfg(feature = "xz")]
pub type XzMap<K, V> = Map<K, V, comde::xz::XzCompressor>;

#[cfg(feature = "deflate")]
pub type DeflateMap<K, V> = Map<K, V, comde::deflate::DeflateCompressor>;

#[cfg(feature = "snappy")]
pub type SnappyMap<K, V> = Map<K, V, comde::snappy::SnappyCompressor>;

pub struct Map<K, V, C>
where
    V: Compress,
    C: Compressor<V>
{
    map: phf_codegen::Map<K>,
    compressor: C,
    #[doc(hidden)]
    __value: std::marker::PhantomData<V>,
}

impl<K: Hash + PhfHash + Eq + FmtConst, V, C> Map<K, V, C>
where
    V: Compress,
    C: Compressor<V>,
{
    pub fn new() -> Map<K, V, C> {
        Map {
            map: phf_codegen::Map::new(),
            compressor: C::new(),
            __value: std::marker::PhantomData::<V>,
        }
    }

    #[inline]
    pub fn entry(&mut self, key: K, value: V) -> &mut Map<K, V, C> {
        let bytes = self.compressor.compress(value).unwrap();
        self.map.entry(key, &format!("{:?}", ByteString::new(bytes)));
        self
    }

    #[inline]
    pub fn phf_path(&mut self, path: &str) -> &mut Map<K, V, C> {
        self.map.phf_path(path);
        self
    }

    delegate! {
        target self.map {
            pub fn build(&self) -> phf_codegen::DisplayMap<K>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "snappy")]
    fn basic_snappy() {
        let mut map = SnappyMap::new();

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let out = map.build();
        println!("{}", out);
    }

    #[test]
    #[cfg(feature = "xz")]
    fn basic_xz() {
        let mut map = XzMap::new();

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let out = map.build();
        println!("{}", out);
    }
}