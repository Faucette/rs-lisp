use ::{Context, Ptr};
use ::lang::{Value, Object, List, Function, Symbol, Scope, Type, Callable};


#[inline]
pub fn eval(context: &Context, scope: Ptr<Object<Scope>>, value: Ptr<Value>) -> Ptr<Value> {
    let typ = value.typ();

    if typ == context.SymbolType {
        let symbol = value.downcast::<Object<Symbol>>().unwrap();

        if let Some(value) = scope.get(symbol.value()) {
            value
        } else {
            context.nil_value.as_value()
        }
    } else if typ == context.ListType {
        eval_list(context, scope, value.downcast::<Object<List>>().unwrap())
    } else {
        value
    }
}

#[inline]
fn eval_list(context: &Context, scope: Ptr<Object<Scope>>, mut list: Ptr<Object<List>>) -> Ptr<Value> {
    let symbol = list.first(context);
    let callable = eval(context, scope, symbol);

    list = list.pop(context);

    eval_fn(context, scope, symbol, callable, list)
}

#[inline]
fn eval_fn(context: &Context, scope: Ptr<Object<Scope>>, symbol: Ptr<Value>, callable: Ptr<Value>, list: Ptr<Object<List>>) -> Ptr<Value> {
    if callable.typ() == context.FunctionType {

        let function = callable.downcast::<Object<Function>>().unwrap();
        let args = eval_arguments(context, scope, list);
        function.call(context, scope, args)

    } else if callable.typ() == context.MacroType {

        let function = callable.downcast::<Object<Function>>().unwrap();
        eval(context, scope, function.call(context, scope, list))

    } else if callable.typ() == context.SpecialFormType {

        let function = callable.downcast::<Object<Function>>().unwrap();
        function.call(context, scope, list)

    } else if callable.typ() == context.TypeType {

        let typ = callable.downcast::<Object<Type>>().unwrap();

        if typ.is_abstract() {
            panic!("can not create abstract type") // TODO throw runtime exception
        } else {
            eval_fn(context, scope, typ.as_value(), typ.constructor.unwrap().as_value(), list)
        }
    } else {
        panic!("can not call {} which is {} as function", symbol, callable)
    }
}

#[inline]
fn eval_arguments(context: &Context, scope: Ptr<Object<Scope>>, mut list: Ptr<Object<List>>) -> Ptr<Object<List>> {
    let mut args = context.gc.new_object(context.ListType, List::new(context));

    while !list.is_empty(context).value() {
        args.push_back_mut(context, eval(context, scope, list.first(context)));
        list = list.pop(context);
    }

    args
}
