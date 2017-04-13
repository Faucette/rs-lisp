use ::{Ptr, Context, eval};
use ::lang::{Value, Object, Scope, List, Type};


#[inline]
pub fn _type(context: &Context, scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut new_args = context.gc.new_object(context.ListType, List::new(context));

    // copy name
    new_args.push_back_mut(context, args.first(context));
    args = args.pop(context);

    // look up type symbol
    let supr = args.first(context);
    if supr.typ() == context.SymbolType {
        let supr_typ = eval(context, scope, supr);
        new_args.push_back_mut(context, supr_typ);
    } else {
        new_args.push_back_mut(context, supr);
    }
    args = args.pop(context);

    // get last argument
    new_args.push_back_mut(context, args.first(context));
    args = args.pop(context);

    Type::constructor(context, scope, new_args)
}
