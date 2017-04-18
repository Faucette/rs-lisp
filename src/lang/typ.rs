use collections::string::String;

use core::{fmt, ptr};
use core::hash::{Hash, Hasher};

use ::Ptr;

use super::function::Function;
use super::object::Object;


pub struct Type {
    pub(crate) name: String,
    pub(crate) constructor: Option<Ptr<Object<Function>>>,
    pub(crate) size: usize,
}

unsafe impl Send for Type {}
unsafe impl Sync for Type {}

impl Hash for Type {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        match self.constructor {
            Some(ref constructor) => Hash::hash(&**constructor, state),
            None => (),
        }
        self.size.hash(state);
    }
}

impl PartialEq for Type {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Eq for Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", &self.name)
    }
}

impl fmt::Debug for Type {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct TypeBuilder {
    name: String,
    constructor: Option<Ptr<Object<Function>>>,
    size: usize,
}

impl TypeBuilder {
    #[inline(always)]
    pub fn new(name: &str) -> Self {
        TypeBuilder {
            name: name.into(),
            constructor: None,
            size: 0usize,
        }
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
    pub fn build(self) -> Type {
        Type {
            name: self.name,
            constructor: self.constructor,
            size: self.size,
        }
    }
}
