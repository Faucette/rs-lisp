use alloc::heap;
use alloc::boxed::Box;

use core::{isize, mem, ptr, fmt};
use core::intrinsics::abort;
use core::ptr::Shared;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{self, AtomicUsize, Ordering};


pub struct CloneBox<T> {
    ptr: *const T,
}

impl<T> CloneBox<T> {

    #[inline(always)]
    pub fn new(value: T) -> Self {
        CloneBox {
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    #[inline(always)]
    pub fn from_ptr(ptr: *mut T) -> Self {
        CloneBox {
            ptr: ptr,
        }
    }
}

impl<T: Clone> Clone for CloneBox<T> {

    #[inline(always)]
    fn clone(&self) -> Self {
        CloneBox {
            ptr: unsafe {
                Box::into_raw(Box::new(
                    (&*self.ptr).clone()
                ))
            },
        }
    }
}

impl<T> Deref for CloneBox<T> {
    type Target = T;


    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T> DerefMut for CloneBox<T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *(self.ptr as *mut T)
        }
    }
}

impl<T> Drop for CloneBox<T> {

    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            heap::deallocate(
                self.ptr as *mut u8,
                mem::size_of::<T>(),
                mem::align_of::<T>()
            );
        }
    }
}
