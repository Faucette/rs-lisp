use ::{Ptr, Context, eval};
use ::lang::{Value, Object, Scope, List, Function};


#[inline]
pub fn _do(context: &Context, scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut result = context.nil_value.as_value();

    while !args.is_empty(context).value() {
        result = eval(context, scope, args.first(context));
        args = args.pop(context);
    }

    result
}
