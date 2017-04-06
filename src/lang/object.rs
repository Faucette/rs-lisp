use collections::string::String;

use core::mem;
use core::ops::{Deref, DerefMut};
use core::hash::{Hash, Hasher};

use collection_traits::*;
use linked_list::LinkedList;

use super::super::GcObject;
use super::super::Ptr;
use super::{Type, TypeBuilder};
use super::Value;
use super::List;
use super::Function;


#[derive(Clone)]
pub struct Object<T> {
    pub(crate) typ: Ptr<Object<Type>>,
    pub(crate) value: Ptr<GcObject<T>>,
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
    pub(crate) fn new_null_typ(value: T) -> Ptr<Self> {
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

impl<T: Function> Function for Object<T> {

    #[inline(always)]
    fn call(&self, args: Ptr<Object<List<Ptr<Value>>>>) -> Ptr<Value> {
        self.value.call(args)
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

impl<T: PartialEq> PartialEq for Object<T> {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        &*self.value == &*other.value
    }
}

impl<T: Eq> Eq for Object<T> {}

impl<T: Hash> Hash for Object<T> {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.value).hash(state);
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

pub fn init_list_typ() -> Ptr<Object<Type>> {
    let mut typ = Object::new_null_typ(
        TypeBuilder::new("Type").is_abstract().build()
    );
    typ.typ = typ;
    typ
}

pub fn init_typs() {
    let mut typ = init_typ_typ();
    let mut any = init_typ(typ, TypeBuilder::new("Any").is_abstract().build());

    typ.value.supr = Some(any);
    any.value.supr = Some(any);

    let list = init_typ(typ, TypeBuilder::new("List")
        .supr(any)
        .size(mem::size_of::<List<Ptr<Value>>>())
        .build());

    let symbol = init_typ(typ, TypeBuilder::new("Symbol")
        .supr(any)
        .size(mem::size_of::<String>())
        .build());

    let number = init_typ(typ, TypeBuilder::new("Number")
        .supr(any).is_abstract().build());

    let real = init_typ(typ, TypeBuilder::new("Real")
        .supr(number).is_abstract().build());

    let float = init_typ(typ, TypeBuilder::new("Float")
        .supr(real).is_abstract().build());

    let integer = init_typ(typ, TypeBuilder::new("Integer")
        .supr(real).is_abstract().build());
    let signed = init_typ(typ, TypeBuilder::new("Signed")
        .supr(integer).is_abstract().build());
    let unsigned = init_typ(typ, TypeBuilder::new("Unsigned")
        .supr(integer).is_abstract().build());

    let boolean = init_typ(typ, TypeBuilder::new("Boolean")
        .supr(integer).size(mem::size_of::<bool>()).is_bits().build());
    let chr = init_typ(typ, TypeBuilder::new("Char")
        .supr(any).size(mem::size_of::<char>()).is_bits().build());

    let int8 = init_typ(typ, TypeBuilder::new("Int8")
        .supr(signed).size(mem::size_of::<i8>()).is_bits().build());
    let int16 = init_typ(typ, TypeBuilder::new("Int16")
        .supr(signed).size(mem::size_of::<i16>()).is_bits().build());
    let int32 = init_typ(typ, TypeBuilder::new("Int32")
        .supr(signed).size(mem::size_of::<i32>()).is_bits().build());
    let int64 = init_typ(typ, TypeBuilder::new("Int64")
        .supr(signed).size(mem::size_of::<i64>()).is_bits().build());

    let uint8 = init_typ(typ, TypeBuilder::new("UInt8")
        .supr(unsigned).size(mem::size_of::<u8>()).is_bits().build());
    let uint16 = init_typ(typ, TypeBuilder::new("UInt16")
        .supr(unsigned).size(mem::size_of::<u16>()).is_bits().build());
    let uint32 = init_typ(typ, TypeBuilder::new("UInt32")
        .supr(unsigned).size(mem::size_of::<u32>()).is_bits().build());
    let uint64 = init_typ(typ, TypeBuilder::new("UInt64")
        .supr(unsigned).size(mem::size_of::<u64>()).is_bits().build());
}