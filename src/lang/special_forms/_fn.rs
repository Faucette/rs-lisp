use ::{Ptr, Context};
use ::lang::{Value, Object, Scope, List, Function};


#[inline]
pub fn _fn(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    Function::constructor(context, scope, args)
}
