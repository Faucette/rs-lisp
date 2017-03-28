use collections::string::String;

use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr::Shared;

use collection_traits::*;
use linked_list::LinkedList;

use super::super::gc::GcObject;
use super::super::utils::CloneBox;
use super::scope::Scope;
use super::typ::{Type, TypeBuilder};
use super::value::Value;


pub struct ObjectInner<T> {
    typ: CloneBox<Object<Type>>,
    value: GcObject<T>,
}


pub struct Object<T> {
    ptr: Shared<ObjectInner<T>>
}

unsafe impl<T: Sync + Send> Send for Object<T> {}
unsafe impl<T: Send + Sync> Sync for Object<T> {}

impl<T> Object<T> {

    #[inline(always)]
    pub fn new(typ: Object<Type>, value: T) -> Self {
        Object {
            ptr: unsafe {
                Shared::new(Box::into_raw(Box::new(ObjectInner {
                    typ: CloneBox::new(typ),
                    value: GcObject::new(value),
                })))
            },
        }
    }

    #[inline(always)]
    fn inner(&self) -> &ObjectInner<T> {
        unsafe {
            &**(self.ptr)
        }
    }
    #[inline(always)]
    fn inner_mut(&mut self) -> &mut ObjectInner<T> {
        unsafe {
            &mut *(*self.ptr as *mut ObjectInner<T>)
        }
    }

    #[inline(always)]
    pub fn value(&self) -> &GcObject<T> {
        &self.inner().value
    }
    #[inline(always)]
    pub fn value_mut(&mut self) -> &mut GcObject<T> {
        &mut self.inner_mut().value
    }
}

impl<T: 'static + Send + Sync> Value for Object<T> {

    #[inline(always)]
    fn typ(&self) -> &Object<Type> {
        &self.inner().typ
    }

    #[inline(always)]
    fn typ_mut(&mut self) -> &mut Object<Type> {
        &mut self.inner_mut().typ
    }
}

impl<T> Clone for Object<T> {

    #[inline(always)]
    fn clone(&self) -> Self {
        Object {
            ptr: self.ptr,
        }
    }
}

impl<T> Deref for Object<T> {
    type Target = T;


    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner().value
    }
}

impl<T> DerefMut for Object<T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_mut().value
    }
}

pub fn init_typ(typ_typ: Object<Type>, typ: Type) -> Object<Type> {
    Object::new(typ_typ, typ)
}

pub unsafe fn init_typ_typ() -> Object<Type> {
    let mut typ = Object::new(
        mem::uninitialized(),
        TypeBuilder::new("Type").is_abstract().build()
    );
    *(typ.inner_mut().typ) = typ.clone();
    typ
}

pub fn init_typs(scope: &mut Scope) {
    let mut typ = unsafe {
        init_typ_typ()
    };
    let mut any = init_typ(typ.clone(), TypeBuilder::new("Any").is_abstract().build());

    scope.define("Any".into(), any);
    scope.define("Type".into(), typ.clone());
}
