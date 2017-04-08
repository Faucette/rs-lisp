use core::fmt;
use core::any::{Any, TypeId};

use super::super::utils::Ptr;
use super::typ::Type;
use super::object::Object;


pub trait Value: Any + Send + Sync {
    fn typ(&self) -> Ptr<Object<Type>>;
}

impl Ptr<Value> {

    #[inline]
    pub fn is<T: Value>(&self) -> bool {
        TypeId::of::<T>() == Any::get_type_id(&**self)
    }

    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: Value>(&self) -> Ptr<T> {
        Ptr::from_ptr((&**self) as *const Value as *const T as *mut T)
    }

    #[inline]
    pub fn downcast<T: Value>(&self) -> Option<Ptr<T>> {
        if self.is::<T>() {
            unsafe {
                Some(self.downcast_ref_unchecked())
            }
        } else {
            None
        }
    }
}
