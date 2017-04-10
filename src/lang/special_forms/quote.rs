use ::{Ptr, Context};
use ::lang::{Value, Object, Scope, List};


#[inline]
pub fn quote(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    args.first(context)
}
