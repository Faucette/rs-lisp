use collections::string::String;

use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr::Shared;

use collection_traits::*;
use linked_list::LinkedList;

use super::super::gc::GcObject;
use super::super::utils::Ptr;
use super::typ::{Type, TypeBuilder};
use super::value::Value;


#[derive(Clone)]
pub struct Object<T> {
    typ: Ptr<Object<Type>>,
    value: Ptr<GcObject<T>>,
}

unsafe impl<T: Sync + Send> Send for Object<T> {}
unsafe impl<T: Send + Sync> Sync for Object<T> {}

impl<T> Object<T> {

    #[inline(always)]
    pub fn new(typ: Ptr<Object<Type>>, value: T) -> Ptr<Self> {
        Ptr::new(Object {
            typ: typ,
            value: Ptr::new(GcObject::new(value)),
        })
    }

    #[inline(always)]
    pub fn new_null_typ(value: T) -> Ptr<Self> {
        Ptr::new(Object {
            typ: unsafe {
                mem::uninitialized()
            },
            value: Ptr::new(GcObject::new(value)),
        })
    }

    #[inline(always)]
    pub fn value(&self) -> &GcObject<T> {
        &self.value
    }
    #[inline(always)]
    pub fn value_mut(&mut self) -> &mut GcObject<T> {
        &mut self.value
    }
}

impl<T: 'static + Send + Sync> Value for Object<T> {

    #[inline(always)]
    fn typ(&self) -> &Object<Type> {
        &*self.typ
    }

    #[inline(always)]
    fn typ_mut(&mut self) -> &mut Object<Type> {
        &mut *self.typ
    }
}

impl<T> Deref for Object<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

impl<T> DerefMut for Object<T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.value
    }
}

pub fn init_typ(typ_typ: Ptr<Object<Type>>, typ: Type) -> Ptr<Object<Type>> {
    Object::new(typ_typ, typ)
}

pub fn init_typ_typ() -> Ptr<Object<Type>> {
    let mut typ = Object::new_null_typ(
        TypeBuilder::new("Type").is_abstract().build()
    );
    typ.typ = typ;
    typ
}

pub fn init_typs() {
    let mut typ = unsafe {
        init_typ_typ()
    };
    let mut any = init_typ(typ, TypeBuilder::new("Any").is_abstract().build());
}
