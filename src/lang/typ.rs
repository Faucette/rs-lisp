use collections::string::String;

use core::{fmt, ptr};

use vector::Vector;

use ::Ptr;

use super::function::Function;
use super::object::Object;


pub struct Type {
    pub(crate) name: String,

    pub(crate) supr: Option<Ptr<Object<Type>>>,

    pub(crate) fields: Option<Vector<String>>,
    pub(crate) types: Option<Vector<Ptr<Object<Type>>>>,

    pub(crate) constructor: Option<Ptr<Object<Function>>>,
    pub(crate) destructor: Option<Ptr<Object<Function>>>,

    pub(crate) size: usize,

    pub(crate) is_abstract: bool,
    pub(crate) is_bits: bool,
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

impl PartialEq for Type {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

pub struct TypeBuilder {
    name: String,

    supr: Option<Ptr<Object<Type>>>,

    fields: Option<Vector<String>>,
    types: Option<Vector<Ptr<Object<Type>>>>,

    constructor: Option<Ptr<Object<Function>>>,
    destructor: Option<Ptr<Object<Function>>>,

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

            fields: None,
            types: None,

            constructor: None,
            destructor: None,

            size: 0usize,

            is_abstract: false,
            is_bits: false,
        }
    }

    #[inline]
    pub fn supr(mut self, supr: Ptr<Object<Type>>) -> Self {
        self.supr = Some(supr);
        self
    }
    #[inline]
    pub fn fields(mut self, fields: Vector<String>) -> Self {
        self.fields = Some(fields);
        self
    }
    #[inline]
    pub fn types(mut self, types: Vector<Ptr<Object<Type>>>) -> Self {
        self.types = Some(types);
        self
    }
    #[inline]
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
    #[inline]
    pub fn constructor(mut self, constructor: Ptr<Object<Function>>) -> Self {
        self.constructor = Some(constructor);
        self
    }
    #[inline]
    pub fn destructor(mut self, destructor: Ptr<Object<Function>>) -> Self {
        self.destructor = Some(destructor);
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

            fields: self.fields,
            types: self.types,

            constructor: self.constructor,
            destructor: self.destructor,

            size: self.size,

            is_abstract: self.is_abstract,
            is_bits: self.is_bits,
        }
    }
}
