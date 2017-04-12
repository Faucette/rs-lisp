use core::fmt;

use ::{Context, Ptr};

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::scope::Scope;


#[derive(PartialEq)]
pub struct Nil;

impl Nil {

    #[inline(always)]
    pub fn new() -> Nil { Nil }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, _args: Ptr<Object<List>>) -> Ptr<Value> {
        context.nil_value.as_value()
    }
}

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
