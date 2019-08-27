use delegate::delegate;
use phf::{PhfHash, map::Map};

use crate::{Compress, Compressor, Decompress, Decompressor};


pub struct CompressedPhfMap<K, V, C, D>
where
    K: 'static,
    V: Compress + Decompress,
    C: Compressor<V>,
    D: Decompressor<V>,
{
    #[doc(hidden)]
    pub map: Map<K, &'static [u8]>,
    #[doc(hidden)]
    pub compressor: C,
    #[doc(hidden)]
    pub decompressor: D,
    #[doc(hidden)]
    pub __value: std::marker::PhantomData<V>,
}

impl<K: PhfHash + Eq, V, C, D> CompressedPhfMap<K, V, C, D>
where
    V: Compress + Decompress,
    C: Compressor<V>,
    D: Decompressor<V>,
{
    // pub const fn new(map: Map<K, &'static [u8]>) -> CompressedPhfMap<K, V, C, D> {
    //     CompressedPhfMap {
    //         map,
    //         compressor: C::new(),
    //         decompressor: D::new(),
    //         __value: std::marker::PhantomData::<V>,
    //     }
    // }
}

#[macro_export]
macro_rules! compressed_phf_map {
    ($compressor:tt, $decompressor:tt, $V:tt, $map:expr) => {
        CompressedPhfMap {
            map: $map,
            compressor: $compressor,
            decompressor: $decompressor,
            __value: std::marker::PhantomData::<$V>,
        }
    }
}
