use collections::string::{String, ToString};

use core::fmt;
use core::any::{Any, TypeId};

use ::Ptr;
use ::lang::{Object, Type};


pub trait Value: Any + fmt::Debug {
    fn typ(&self) -> Ptr<Object<Type>>;
}

impl Ptr<Value> {

    #[inline(always)]
    pub fn is<T: Value>(&self) -> bool {
        TypeId::of::<T>() == Any::get_type_id(&**self)
    }

    #[inline(always)]
    pub unsafe fn downcast_ref_unchecked<T: Value>(&self) -> Ptr<T> {
        Ptr::from_ptr((&**self) as *const Value as *const T as *mut T)
    }

    #[inline(always)]
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

impl fmt::Display for Ptr<Value> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Debug for Ptr<Value> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
