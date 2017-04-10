use ::{Ptr, Context, eval};
use ::lang::{Value, Object, Scope, List, Symbol};


#[inline]
pub fn _let(context: &Context, scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut new_scope =
        context.gc.new_object(context.ScopeType, Scope::new(None, Some(scope)));

    let let_value = args.first(context);

    if let_value.typ() == context.ListType {
        let mut let_list = let_value.downcast::<Object<List>>().unwrap();

        args = args.pop(context);

        while !let_list.is_empty(context).value() {
            let symbol = let_list.first(context);
            let_list = let_list.pop(context);

            let value = eval(context, scope, let_list.first(context));
            let_list = let_list.pop(context);

            if symbol.typ() == context.SymbolType {
                let symbol = symbol.downcast::<Object<Symbol>>().unwrap();
                new_scope.set(symbol.value().clone(), value);
            } else {
                panic!("invalid symbol in let block"); // TODO throw runtime exception
            }
        }
    }

    eval(context, scope, args.first(context))
}
