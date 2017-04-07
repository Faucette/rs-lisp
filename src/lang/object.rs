use core::mem;
use core::ops::{Deref, DerefMut};
use core::hash::{Hash, Hasher};

use super::super::Ptr;
use super::Type;
use super::Value;
use super::List;
use super::Function;


#[derive(Clone)]
pub struct Object<T> {
    pub(crate) typ: Ptr<Object<Type>>,
    pub(crate) value: T,
}

unsafe impl<T: Sync + Send> Send for Object<T> {}
unsafe impl<T: Send + Sync> Sync for Object<T> {}

impl<T> Object<T> {

    #[inline(always)]
    pub fn new(typ: Ptr<Object<Type>>, value: T) -> Ptr<Self> {
        Ptr::new(Object {
            typ: typ,
            value: value,
        })
    }

    #[inline(always)]
    pub(crate) fn new_null_typ(value: T) -> Ptr<Self> {
        Ptr::new(Object {
            typ: unsafe {
                mem::uninitialized()
            },
            value: value,
        })
    }
}

impl<T: Send + Sync + 'static> Ptr<Object<T>> {

    #[inline(always)]
    pub fn as_value(&self) -> Ptr<Value> {
        unsafe {
            Ptr::from_ptr(self.as_ptr() as *mut Value)
        }
    }
}

impl<T: 'static + Send + Sync> Value for Object<T> {

    #[inline(always)]
    fn typ(&self) -> Ptr<Object<Type>> {
        self.typ
    }
}

impl<T: Function> Function for Object<T> {

    #[inline(always)]
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value> {
        self.value.call(args)
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

impl<T: Hash> Hash for Object<T> {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
