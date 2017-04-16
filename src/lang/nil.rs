use core::fmt;
use core::hash::{self, Hasher};

use ::{Context, Hash, Ptr};

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::scope::Scope;


#[derive(Hash)]
pub struct Nil;

impl Nil {

    #[inline(always)]
    pub fn new() -> Nil { Nil }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, _args: Ptr<Object<List>>) -> Ptr<Value> {
        context.nil_value.as_value()
    }
}

impl Hash for Nil {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(self, state);
    }
}

impl PartialEq for Nil {

    #[inline(always)]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Nil {}

impl fmt::Display for Nil {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nil")
    }
}

impl fmt::Debug for Nil {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
