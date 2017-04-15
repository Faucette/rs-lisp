use alloc::boxed::Box;
use alloc::heap;

use core::{fmt, mem};

use collection_traits::*;
use linked_list::LinkedList;

use ::Ptr;
use ::lang::{Object, Type, Value};


pub struct EmbedGc {
    total: *mut usize,
    list: *mut LinkedList<*mut Value>
}

impl EmbedGc {

    #[inline(always)]
    pub fn new() -> Self {
        EmbedGc {
            total: Box::into_raw(Box::new(0usize)),
            list: Box::into_raw(Box::new(LinkedList::new())),
        }
    }

    pub fn total(&self) -> usize {
        unsafe {*self.total}
    }

    #[inline]
    pub fn new_object<T: 'static + fmt::Debug>(&self, typ: Ptr<Object<Type>>, value: T) -> Ptr<Object<T>> {
        let value = Box::into_raw(Box::new(Object::new(typ, value)));


        *(unsafe {&mut *self.total}) += mem::size_of::<T>();
        (unsafe {&mut *self.list}).push_back(value);

        unsafe {
            Ptr::from_ptr(value)
        }
    }

    #[inline(always)]
    pub fn new_null_typ_object<T: 'static + fmt::Debug>(&self, value: T) -> Ptr<Object<T>> {
        self.new_object(unsafe {mem::uninitialized()}, value)
    }
}

impl Drop for EmbedGc {

    #[inline]
    fn drop(&mut self) {
        unsafe {
            heap::deallocate(
                self.total as *mut u8,
                mem::size_of::<usize>(),
                mem::align_of::<usize>()
            );
            heap::deallocate(
                self.list as *mut u8,
                mem::size_of::<LinkedList<*mut Value>>(),
                mem::align_of::<LinkedList<*mut Value>>()
            );
        }
    }
}
