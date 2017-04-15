use core::mem;
use core::hash::Hash;

use hash_map::{DefaultHasher};


pub trait LHash {
    fn hash(&self, &mut DefaultHasher);
}

impl LHash for f32 {
    fn hash(&self, hasher: &mut DefaultHasher) {
        Hash::hash(unsafe {
            &mem::transmute::<_, u32>(*self)
        }, hasher);
    }
}

impl LHash for f64 {
    fn hash(&self, hasher: &mut DefaultHasher) {
        Hash::hash(unsafe {
            &mem::transmute::<_, u64>(*self)
        }, hasher);
    }
}
