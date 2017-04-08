use ::{Context, Ptr};
use ::lang::{Value, Object, List, Nil, Symbol, Scope};


pub fn eval(context: &Context, scope: Ptr<Object<Scope>>, value: Ptr<Value>) -> Ptr<Value> {
    let typ = value.typ();

    if typ == context.SymbolType {
        let symbol = value.downcast::<Object<Symbol>>().unwrap();

        if let Some(value) = scope.get(symbol.value()) {
            value
        } else {
            context.gc.new_object(context.NilType, Nil::new()).as_value()
        }
    } else if typ == context.ListType {
        eval_list(context, scope, value.downcast::<Object<List>>().unwrap())
    } else {
        value
    }
}

fn eval_list(context: &Context, scope: Ptr<Object<Scope>>, mut list: Ptr<Object<List>>) -> Ptr<Value> {
    let callable = eval(context, scope, list.first(context));
    list = list.pop(context);

    if callable.typ() == context.FunctionType {
        list.as_value()
    } else {
        context.gc.new_object(context.NilType, Nil::new()).as_value()
    }
}
