use alloc::boxed::Box;

use core::{fmt, ptr};
use core::ops::{Deref, DerefMut};


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
}

impl<T: ?Sized> Ptr<T> {

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

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        unsafe {
            &*(self.ptr as *const T)
        }
    }

    #[inline(always)]
    pub fn as_mut(&mut self) -> &mut T {
        unsafe {
            &mut *(self.ptr as *mut T)
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<T: fmt::Display> fmt::Display for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<T: ?Sized> Deref for Ptr<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> DerefMut for Ptr<T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: ?Sized> Clone for Ptr<T> {

    #[inline(always)]
    fn clone(&self) -> Self {
        Ptr {
            ptr: self.ptr,
        }
    }
}

impl<T: ?Sized> Copy for Ptr<T> {}

impl<T: ?Sized + PartialEq> PartialEq for Ptr<T> {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            &*self.ptr == &*other.ptr
        }
    }
}

impl<T: ?Sized + Eq> Eq for Ptr<T> {}
