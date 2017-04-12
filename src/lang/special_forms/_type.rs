use ::{Ptr, Context};
use ::lang::{Value, Object, Scope, List, Type};


#[inline]
pub fn _type(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    Type::constructor(context, scope, args)
}
