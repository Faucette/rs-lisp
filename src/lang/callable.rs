use alloc::boxed::Box;

use ::Ptr;
use ::Context;

use super::value::Value;
use super::object::Object;
use super::list::List;


pub trait Callable {
    fn call(&self, &Context, Ptr<Object<List>>) -> Ptr<Value>;
}

impl Callable for fn(&Context, Ptr<Object<List>>) -> Ptr<Value> {

    #[inline(always)]
    fn call(&self, context: &Context, args: Ptr<Object<List>>) -> Ptr<Value> {
        (self)(context, args)
    }
}
