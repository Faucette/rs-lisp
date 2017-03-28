use alloc::boxed::Box;
use collections::string::String;

use core::mem;

use vector::Vector;

use super::value::Value;
use super::object::Object;


pub struct Type {
    name: String,

    supr: Option<Object<Type>>,

    names: Option<Vector<String>>,
    types: Option<Vector<Object<Type>>>,

    constructor: Option<usize>,
    destructor: Option<usize>,

    size: usize,

    is_abstract: bool,
    is_bits: bool,
}

unsafe impl Send for Type {}
unsafe impl Sync for Type {}

impl Type {

    #[inline(always)]
    pub fn is_real(&self) -> bool { !self.is_abstract }

    #[inline(always)]
    pub fn is_abstract(&self) -> bool { self.is_abstract }

    #[inline(always)]
    pub fn is_bits(&self) -> bool { self.is_bits }
}


pub struct TypeBuilder {
    name: String,

    supr: Option<Object<Type>>,

    names: Option<Vector<String>>,
    types: Option<Vector<Object<Type>>>,

    constructor: Option<usize>,
    destructor: Option<usize>,

    size: usize,

    is_abstract: bool,
    is_bits: bool,
}

impl TypeBuilder {
    #[inline(always)]
    pub fn new(name: &str) -> Self {
        TypeBuilder {
            name: name.into(),

            supr: None,

            names: None,
            types: None,

            constructor: None,
            destructor: None,

            size: 0,

            is_abstract: false,
            is_bits: false,
        }
    }

    #[inline]
    pub fn supr(mut self, supr: Option<Object<Type>>) -> Self {
        self.supr = supr;
        self
    }
    #[inline]
    pub fn names(mut self, names: Vector<String>) -> Self {
        self.names = Some(names);
        self
    }
    #[inline]
    pub fn types(mut self, types: Vector<Object<Type>>) -> Self {
        self.types = Some(types);
        self
    }
    #[inline]
    pub fn constructor(mut self, constructor: Option<usize>) -> Self {
        self.constructor = constructor;
        self
    }
    #[inline]
    pub fn destructor(mut self, destructor: Option<usize>) -> Self {
        self.destructor = destructor;
        self
    }
    #[inline]
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
    #[inline]
    pub fn is_abstract(mut self) -> Self {
        self.is_abstract = true;
        self
    }
    #[inline]
    pub fn is_bits(mut self) -> Self {
        self.is_bits = true;
        self
    }
    #[inline]
    pub fn build(self) -> Type {
        Type {
            name: self.name,

            supr: self.supr,

            names: self.names,
            types: self.types,

            constructor: self.constructor,
            destructor: self.destructor,

            size: self.size,

            is_abstract: self.is_abstract,
            is_bits: self.is_bits,
        }
    }
}
