use ::{Ptr, Context, eval};
use ::lang::{Value, Object, Scope, List, Keyword, Symbol};


#[inline]
pub fn quote(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    args.first(context)
}
