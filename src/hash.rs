use collections::string::String;
use collections::vec::Vec;
use collections::linked_list::LinkedList;

use core::mem;
use core::hash::{self, Hasher};
use core::sync::atomic::{AtomicPtr, Ordering};


pub trait Hash {
    fn hash<H: Hasher>(&self, &mut H);
}

macro_rules! impl_Hash {
    ($($t:ident),*) => (
        $(impl Hash for $t {

            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {
                hash::Hash::hash(self, state);
            }
        })*
    );
}
impl_Hash!(
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
    char, bool, str, String
);


impl<T: Hash> Hash for Option<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &Some(ref value) => value.hash(state),
            &None => (),
        }
    }
}

impl<T: Hash> Hash for Vec<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in self.iter() {
            Hash::hash(value, state);
        }
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in self.iter() {
            Hash::hash(value, state);
        }
    }
}

impl<T: Hash> Hash for AtomicPtr<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (unsafe { &*self.load(Ordering::Relaxed) }).hash(state);
    }
}

impl<T: Hash> Hash for [T; 32] {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in self.iter() {
            Hash::hash(value, state);
        }
    }
}


impl Hash for f32 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(unsafe {
            &mem::transmute::<f32, u32>(*self)
        }, state);
    }
}
impl Hash for f64 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(unsafe {
            &mem::transmute::<f64, u64>(*self)
        }, state);
    }
}
