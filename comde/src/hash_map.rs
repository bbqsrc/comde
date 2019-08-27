use std::collections::{HashMap, hash_map::RandomState};
use std::hash::Hash;
use std::borrow::Borrow;

use delegate::delegate;

use crate::{Compress, Compressor, Decompress, Decompressor};

pub struct CompressedHashMap<K, V, R, C, D>
where
    V: Compress + Decompress,
    C: Compressor<V>,
    D: Decompressor<V>,
{
    map: HashMap<K, Vec<u8>, R>,
    compressor: C,
    decompressor: D,
    #[doc(hidden)]
    __value: std::marker::PhantomData<V>,
}

impl<K: Hash + Eq, V, C, D> CompressedHashMap<K, V, RandomState, C, D>
where
    V: Compress + Decompress,
    C: Compressor<V>,
    D: Decompressor<V>,
{
    pub fn new() -> CompressedHashMap<K, V, RandomState, C, D> {
        CompressedHashMap {
            map: HashMap::new(),
            compressor: C::new(),
            decompressor: D::new(),
            __value: std::marker::PhantomData::<V>,
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let bytes = self.compressor.compress(v).unwrap();
        self.map
            .insert(k, bytes)
            .map(|x| self.decompressor.decompress(&*x).unwrap())
    }

    #[inline(always)]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map
            .get(k)
            .map(|x| self.decompressor.decompress(&*x).unwrap())
    }

    delegate! {
        target self.map {
            #[target_method(get)]
            pub fn get_raw<Q: ?Sized>(&self, k: &Q) -> Option<&Vec<u8>>
            where
                K: Borrow<Q>,
                Q: Hash + Eq;
        }
    }
}
