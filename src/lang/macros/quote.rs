use ::{Ptr, Context, eval};
use ::lang::{Value, Object, Scope, List, Keyword, Symbol};


#[inline]
pub fn quote(_context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    args.as_value()
}
