use ::Ptr;
use ::Context;

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::scope::Scope;


pub trait Callable {
    fn call(&self, &Context, Ptr<Object<Scope>>, Ptr<Object<List>>) -> Ptr<Value>;
}

impl Callable for fn(&Context, Ptr<Object<Scope>>, Ptr<Object<List>>) -> Ptr<Value> {

    #[inline(always)]
    fn call(&self, context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        (self)(context, scope, args)
    }
}
