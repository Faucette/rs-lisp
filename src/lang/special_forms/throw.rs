use ::{Ptr, Context};
use ::lang::{Value, Object, Scope, List};


#[inline]
pub fn throw(context: &Context, _scope: Ptr<Object<Scope>>, _args: Ptr<Object<List>>) -> Ptr<Value> {
    context.nil_value.as_value() // TODO thro runtime error
}
