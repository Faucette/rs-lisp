use alloc::heap;
use alloc::boxed::Box;

use core::{isize, mem, ptr, fmt};
use core::intrinsics::abort;
use core::ptr::Shared;
use core::ops::{Deref, DerefMut};
use core::hash::{Hash, Hasher};


pub struct GcObject<T: ?Sized> {
    ptr: Shared<T>,
}

unsafe impl<T: ?Sized + Sync + Send> Send for GcObject<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for GcObject<T> {}

impl<T> GcObject<T> {

    #[inline(always)]
    pub fn new(data: T) -> Self {
        unsafe {
            Self::from_ptr(Box::into_raw(Box::new(data)))
        }
    }

    #[inline(always)]
    pub unsafe fn from_ptr(ptr: *mut T) -> Self {
        GcObject {
            ptr: Shared::new(ptr),
        }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        *self.ptr as *mut T
    }
}

impl<T: fmt::Debug> fmt::Debug for GcObject<T> {

    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            fmt::Debug::fmt(&**self.ptr, f)
        }
    }
}

impl<T> GcObject<T> {

    #[inline]
    fn drop_slow(&mut self) {
        unsafe {
            let p = *self.ptr as *mut T;
            heap::deallocate(p as *mut u8, mem::size_of::<T>(), mem::align_of::<T>());
        }
    }
}

impl<T> Clone for GcObject<T> {

    #[inline]
    fn clone(&self) -> Self {
        GcObject {
            ptr: self.ptr
        }
    }
}

impl<T> Deref for GcObject<T> {
    type Target = T;


    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe {
            &**self.ptr
        }
    }
}

impl<T> DerefMut for GcObject<T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *(*self.ptr as *mut T)
        }
    }
}

impl<T: PartialEq> PartialEq for GcObject<T> {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        &*self.ptr == &*other.ptr
    }
}

impl<T: Eq> Eq for GcObject<T> {}

impl<T: Hash> Hash for GcObject<T> {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.ptr).hash(state);
    }
}
