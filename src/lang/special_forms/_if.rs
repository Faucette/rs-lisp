use ::{Ptr, Context, eval};
use ::lang::{Value, Object, Scope, List};


#[inline]
pub fn _if(context: &Context, scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let expr = eval(context, scope, args.first(context));

    args = args.pop(context);

    if expr.typ() == context.BooleanType && expr.downcast::<Object<bool>>().unwrap().value() == &true {
        eval(context, scope, args.first(context))
    } else {
        args = args.pop(context);
        eval(context, scope, args.first(context))
    }
}
