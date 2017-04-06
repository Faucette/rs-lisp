use alloc::heap;

use core::{mem, ptr};
use core::ops::{Deref, DerefMut};
use core::hash::{Hash, Hasher};


pub struct Ptr<T: ?Sized> {
    ptr: *mut T,
}

unsafe impl<T: Sync + Send> Send for Ptr<T> {}
unsafe impl<T: Send + Sync> Sync for Ptr<T> {}

impl<T> Ptr<T> {

    #[inline(always)]
    pub fn new(value: T) -> Self {
        Ptr {
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    #[inline(always)]
    pub const fn null() -> Self {
        Ptr {
            ptr: ptr::null::<T>() as *mut T,
        }
    }

    #[inline(always)]
    pub unsafe fn from_ptr(value: *mut T) -> Self {
        Ptr {
            ptr: value,
        }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe {
            &*(self.ptr as *const T)
        }
    }
}

impl<T> DerefMut for Ptr<T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *(self.ptr as *mut T)
        }
    }
}

impl<T> Clone for Ptr<T> {

    #[inline(always)]
    fn clone(&self) -> Self {
        Ptr {
            ptr: self.ptr,
        }
    }
}

impl<T> Copy for Ptr<T> {}

impl<T: PartialEq> PartialEq for Ptr<T> {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            &*self.ptr == &*other.ptr
        }
    }
}

impl<T: Hash> Hash for Ptr<T> {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            (&*self.ptr).hash(state);
        }
    }
}

impl<T: Eq> Eq for Ptr<T> {}
