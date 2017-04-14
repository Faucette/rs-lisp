use ::{Context, Ptr};
use ::lang::{Value, Object, List, Function, Symbol, Scope, Type, Callable};


#[derive(Debug)]
pub enum State {
    Eval,
    List,
    EvalList,
    Done,
    None,
}


#[inline]
pub fn eval_recur(context: &Context, scope: Ptr<Object<Scope>>, value: Ptr<Value>) -> Ptr<Value> {
    let mut state = State::Eval;
    let mut next_state = State::None;
    let mut return_value = value;

    let mut stack: Ptr<Object<List>> = context.gc.new_object(context.ListType, List::new(context));
    stack.push_back_mut(context, value);

    loop {
        match state {

            State::None => break,

            State::Done => {
                match next_state {
                    State::None => break,
                    _ => {
                        state = next_state;
                        next_state = State::None;
                    },
                }
            },

            State::Eval => if stack.is_empty(context).value() == &false {
                let value = stack.first(context);
                stack = stack.pop(context);

                let typ = value.typ();

                if typ == context.SymbolType {
                    let symbol = value.downcast::<Object<Symbol>>().unwrap();

                    if let Some(value) = scope.get(symbol.value()) {
                        stack = stack.push(context, value);
                    } else {
                        stack = stack.push(context, context.nil_value.as_value());
                    }

                    state = State::Done;
                } else if typ == context.ListType {
                    stack = stack.push(context, value);
                    state = State::List;
                } else {
                    return_value = value;
                }
            } else {
                state = State::Done;
            },

            State::List => {
                let mut list = stack.first(context).downcast::<Object<List>>().unwrap();
                stack = stack.pop(context);

                let value = list.first(context);
                list = list.pop(context);

                stack = stack.push(context, list.as_value());
                stack = stack.push(context, value);
                stack = stack.push(context, value);
                state = State::Eval;
                next_state = State::EvalList;
            }

            State::EvalList => {
                let callable = stack.first(context);
                stack = stack.pop(context);

                let symbol = stack.first(context);
                stack = stack.pop(context);

                let mut list = stack.first(context).downcast::<Object<List>>().unwrap();
                stack = stack.pop(context);

                if callable.typ() == context.FunctionType {

                    let function = callable.downcast::<Object<Function>>().unwrap();
                    let mut args = context.gc.new_object(context.ListType, List::new(context));

                    while !list.is_empty(context).value() {
                        args.push_back_mut(context, eval_recur(context, scope, list.first(context)));
                        list = list.pop(context);
                    }

                    return_value = function.call(context, scope, args);
                    state = State::Done;
                    next_state = State::None;

                } else if callable.typ() == context.MacroType {

                    let function = callable.downcast::<Object<Function>>().unwrap();

                    stack = stack.push(context, function.call(context, scope, list));
                    state = State::Eval;
                    next_state = State::None;

                } else if callable.typ() == context.SpecialFormType {

                    let function = callable.downcast::<Object<Function>>().unwrap();

                    return_value = function.call(context, scope, list);
                    state = State::Done;
                    next_state = State::None;

                } else if callable.typ() == context.TypeType {

                    let typ = callable.downcast::<Object<Type>>().unwrap();

                    if typ.is_abstract() {
                        panic!("can not create abstract type") // TODO throw runtime exception
                    } else {
                        let typ_value = typ.as_value();
                        stack = stack.push(context, list.as_value());
                        stack = stack.push(context, typ_value);
                        stack = stack.push(context, typ_value);
                        state = State::Eval;
                        next_state = State::EvalList;
                    }
                } else {
                    panic!("can not call {} which is {} as function", symbol, callable)
                }
            },
        }
    }

    return_value
}
