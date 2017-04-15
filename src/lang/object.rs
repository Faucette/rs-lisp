use core::fmt;
use core::ops::{Deref, DerefMut};
use core::hash::{Hasher};

use hash_map::DefaultHasher;

use ::{LHash, Ptr};
use ::lang::{Value, Type};


pub struct Object<T> {
    pub(crate) typ: Ptr<Object<Type>>,
    pub(crate) value: T,
}

unsafe impl<T> Send for Object<T> {}
unsafe impl<T> Sync for Object<T> {}

impl<T> Object<T> {

    #[inline(always)]
    pub fn new(typ: Ptr<Object<Type>>, value: T) -> Self {
        Object {
            typ: typ,
            value: value,
        }
    }

    #[inline(always)]
    pub fn value(&self) -> &T {
        &self.value
    }
    #[inline(always)]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: 'static + fmt::Debug> Ptr<Object<T>> {

    #[inline(always)]
    pub fn as_value(&self) -> Ptr<Value> {
        unsafe {
            Ptr::from_ptr(self.as_ptr() as *mut Value)
        }
    }
}

impl<T: 'static + LHash + fmt::Debug> Value for Object<T> {

    #[inline(always)]
    fn typ(&self) -> Ptr<Object<Type>> {
        self.typ
    }
    fn hash(&self, hasher: &mut DefaultHasher) {
        self.value.hash(hasher);
    }
}

impl<T: fmt::Display> fmt::Display for Object<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value(), f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Object<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.value(), f)
    }
}

impl<T> Deref for Object<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Object<T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: PartialEq> PartialEq for Object<T> {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        &self.value == &other.value
    }
}

impl<T: Eq> Eq for Object<T> {}

impl<T: LHash> LHash for Object<T> {

    #[inline(always)]
    fn hash(&self, state: &mut DefaultHasher) {
        self.value.hash(state);
    }
}
