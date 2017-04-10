use ::{Ptr, Context};
use ::lang::{Value, Object, Scope, List, Function};


#[inline]
pub fn _macro(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    Function::macro_constructor(context, scope, args)
}
