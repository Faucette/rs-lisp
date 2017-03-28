use alloc::heap;
use alloc::boxed::Box;

use core::{isize, mem, ptr, fmt};
use core::intrinsics::abort;
use core::ptr::Shared;
use core::ops::{Deref, DerefMut};


pub struct GcObject<T: ?Sized> {
    ptr: Shared<T>,
}

unsafe impl<T: ?Sized + Sync + Send> Send for GcObject<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for GcObject<T> {}

impl<T> GcObject<T> {

    #[inline(always)]
    pub fn new(data: T) -> Self {
        GcObject {
            ptr: unsafe {
                Shared::new(Box::into_raw(Box::new(data)))
            },
        }
    }

    #[inline(always)]
    pub fn from_ptr(ptr: *mut T) -> Self {
        GcObject {
            ptr: unsafe {
                Shared::new(ptr)
            },
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
