use std::borrow::Borrow;

use phf::{map::Map, PhfHash};

use crate::{Decompress, Decompressor};

pub struct CompressedPhfMap<K, V, D>
where
    K: 'static,
    V: Decompress,
    D: Decompressor,
{
    #[doc(hidden)]
    pub map: Map<K, &'static [u8]>,
    #[doc(hidden)]
    pub value_ty: std::marker::PhantomData<V>,
    pub decompressor_ty: std::marker::PhantomData<D>,
}

impl<K: PhfHash + Eq, V, D> CompressedPhfMap<K, V, D>
where
    V: Decompress,
    D: Decompressor,
{
    pub fn get<T: ?Sized>(&self, key: &T) -> Option<V>
    where
        T: Eq + PhfHash,
        K: Borrow<T>,
    {
        self.map
            .get(key)
            .map(|value| D::new().from_reader(std::io::Cursor::new(value)).unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let phf_map = crate::phf::CompressedPhfMap {
            map: ::phf::Map {
                key: 3213172566270843353,
                disps: ::phf::Slice::Static(&[
                    (0, 0),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("boop", b"(\xb5/\xfd\x00\x80\r\x01\x00\xb0this is a stringindeed\x03\x00^\xd5\xba\xea\x05\x8a,K"),
                ]),
            },
            decompressor_ty: std::marker::PhantomData::<crate::zstd::ZstdDecompressor>,
            value_ty: std::marker::PhantomData::<String>
        };
        let result = phf_map.get("boop");
        println!("{:?}", result);
        assert_eq!(Some("this is a string string string string string string this is indeed a string string string".to_string()), result);
    }
}
