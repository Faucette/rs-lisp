use core::{fmt, mem};
use core::hash::{self, Hasher};
use core::any::{Any, TypeId};

use hash_map::DefaultHasher;

use ::{Hash, Ptr};
use ::lang::{Object, Type};


pub trait Value: Any + fmt::Debug {
    fn typ(&self) -> Ptr<Object<Type>>;
    fn equals(&self, Ptr<Value>) -> bool;
    fn hash(&self, &mut DefaultHasher);
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


impl Hash for Ptr<Value> {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Value::hash(&**self, unsafe {
            mem::transmute::<_, &mut DefaultHasher>(state)
        });
    }
}
impl hash::Hash for Ptr<Value> {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self, state);
    }
}

impl PartialEq for Ptr<Value> {

    #[inline(always)]
    fn eq(&self, other: &Ptr<Value>) -> bool {
        self.equals(*other)
    }
}

impl Eq for Ptr<Value> {}

impl fmt::Display for Ptr<Value> {

    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Debug for Ptr<Value> {

    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
