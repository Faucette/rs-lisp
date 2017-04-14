use collection_traits::*;
use linked_list::LinkedList;

use ::{Context, Ptr};
use ::lang::{Value, Object, List, Function, Symbol, Struct, Scope, Type, Callable};


#[derive(Debug)]
pub enum State {
    Eval,
    EvalList,
    EvalArguments,
    EvalFunction,
    EvalMacro,
    FunctionReturn,
    Def,
    Do,
    Fn,
    If,
    Let,
    Macro,
    Quote,
    Type,
    Throw,
    Done,
}


#[inline]
pub fn eval_recur(context: &Context, scope: Ptr<Object<Scope>>, value: Ptr<Value>) -> Ptr<Value> {
    let mut scopes: LinkedList<Ptr<Object<Scope>>> = LinkedList::new();
    scopes.push_front(scope);

    let mut states: LinkedList<State> = LinkedList::new();
    states.push_front(State::Eval);

    let mut stack: LinkedList<Ptr<Value>> = LinkedList::new();
    stack.push_front(value);

    loop {

        match states.pop_front() {

            Some(state) => match state {

                State::Done => break,

                State::Eval => {
                    let value = stack.pop_front().unwrap();
                    let typ = value.typ();

                    if typ == context.SymbolType {
                        let symbol = value.downcast::<Object<Symbol>>().unwrap();

                        if let Some(value) = scopes.front().unwrap().get(symbol.value()) {
                            stack.push_front(value);
                        } else {
                            stack.push_front(context.nil_value.as_value());
                        }
                    } else if typ == context.ListType {
                        let mut list = value.downcast::<Object<List>>().unwrap();
                        let first = list.first(context);

                        match first.downcast::<Object<Symbol>>() {
                            Some(symbol) => {
                                list = list.pop(context);

                                match (&**symbol).as_ref() {
                                    "def" => {
                                        let symbol = list.first(context);
                                        list = list.pop(context);

                                        let value = list.first(context);
                                        //list = list.pop(context);

                                        stack.push_front(symbol);
                                        stack.push_front(value);

                                        states.push_front(State::Def);
                                        states.push_front(State::Eval);
                                    },
                                    "do" => {
                                        let first = list.first(context);
                                        list = list.pop(context);

                                        stack.push_front(list.as_value());
                                        stack.push_front(first);
                                        states.push_front(State::Do);
                                        states.push_front(State::Eval);
                                    },
                                    "fn" => {
                                        stack.push_front(list.as_value());
                                        states.push_front(State::Fn);
                                    },
                                    "if" => {
                                        let expr = list.first(context);
                                        list = list.pop(context);

                                        let if_statement = list.first(context);
                                        list = list.pop(context);

                                        let else_statement = list.first(context);
                                        //list = list.pop(context);

                                        stack.push_front(else_statement);
                                        stack.push_front(if_statement);
                                        stack.push_front(expr);

                                        states.push_front(State::If);
                                        states.push_front(State::Eval);
                                    },
                                    "let" => {
                                        stack.push_front(list.as_value());
                                        states.push_front(State::Let);
                                    },
                                    "macro" => {
                                        stack.push_front(list.as_value());
                                        states.push_front(State::Macro);
                                    },
                                    "quote" => {
                                        stack.push_front(list.first(context));
                                        states.push_front(State::Quote);
                                    },
                                    "type" => {
                                        stack.push_front(list.as_value());
                                        states.push_front(State::Type);
                                    },
                                    "throw" => {
                                        stack.push_front(list.as_value());
                                        states.push_front(State::Throw);
                                    },
                                    _ => {
                                        stack.push_front(list.as_value());
                                        stack.push_front(first);
                                        stack.push_front(first);
                                        states.push_front(State::EvalList);
                                        states.push_front(State::Eval);
                                    }
                                }
                            },
                            None => {
                                // TODO throw error?
                                stack.push_front(value);
                            },
                        }
                    } else {
                        stack.push_front(value);
                    }
                },

                State::EvalList => {
                    let callable = stack.pop_front().unwrap();
                    let symbol = stack.pop_front().unwrap();
                    let args = stack.pop_front().unwrap();
                    let callable_typ = callable.typ();

                    if callable_typ == context.FunctionType {
                        let args = args.downcast::<Object<List>>().unwrap();
                        let first = args.first(context);
                        let eval_args = context.gc.new_object(context.ListType, List::new(context));

                        stack.push_front(callable);
                        stack.push_front(eval_args.as_value());
                        stack.push_front(args.as_value());
                        stack.push_front(first);
                        states.push_front(State::EvalFunction);
                        states.push_front(State::EvalArguments);
                        states.push_front(State::Eval);
                    } else if callable_typ == context.MacroType {
                        let args = args.downcast::<Object<List>>().unwrap();

                        stack.push_front(callable);
                        stack.push_front(args.as_value());
                        states.push_front(State::EvalMacro);
                    } else {
                        stack.push_front(context.nil_value.as_value()); // TODO throw error
                    }
                },

                State::EvalArguments => {
                    let mut eval_arg = stack.pop_front().unwrap();
                    let mut args = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let mut eval_args = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();

                    eval_args = eval_args.push(context, eval_arg);
                    stack.push_front(eval_args.as_value());

                    if !args.is_empty(context).value() {
                        let first = args.first(context);
                        args = args.pop(context);

                        stack.push_front(args.as_value());
                        stack.push_front(first);
                        states.push_front(State::EvalArguments);
                        states.push_front(State::Eval);
                    }
                },

                State::EvalMacro => {
                    let args = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let callable = stack.pop_front().unwrap().downcast::<Object<Function>>().unwrap();
                    let scope = callable.get_scope(context, *scopes.front().unwrap(), args);

                    match &**callable {
                        &Function::Internal(_, _, _, body) => {
                            scopes.push_front(scope);
                            stack.push_front(body);
                            states.push_front(State::Eval);
                            states.push_front(State::FunctionReturn);
                            states.push_front(State::Eval);
                        },
                        &Function::Constructor(typ) => {
                            stack.push_front(Struct::constructor(context, typ, args));
                            states.push_front(State::Eval);
                        },
                        &Function::Rust(ref fn_ptr) => {
                            stack.push_front(Callable::call(fn_ptr, context, scope, args));
                            states.push_front(State::Eval);
                        },
                    }
                },

                State::EvalFunction => {
                    let args = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let callable = stack.pop_front().unwrap().downcast::<Object<Function>>().unwrap();
                    let scope = callable.get_scope(context, *scopes.front().unwrap(), args);

                    match &**callable {
                        &Function::Internal(_, _, _, body) => {
                            scopes.push_front(scope);
                            stack.push_front(body);
                            states.push_front(State::FunctionReturn);
                            states.push_front(State::Eval);
                        },
                        &Function::Constructor(typ) => {
                            stack.push_front(Struct::constructor(context, typ, args));
                        },
                        &Function::Rust(ref fn_ptr) => {
                            stack.push_front(Callable::call(fn_ptr, context, scope, args));
                        },
                    }
                },
                State::FunctionReturn => {
                    scopes.pop_front();
                }

                State::Def => {
                    let mut value = stack.pop_front().unwrap();
                    let mut symbol = stack.pop_front().unwrap();

                    let mut scope = scopes.front_mut().unwrap();

                    scope.set(&Struct::key_to_string(context, &symbol), value);

                    stack.push_front(value);
                },
                State::Do => {
                    let value = stack.pop_front().unwrap();
                    let mut list = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();

                    if !list.is_empty(context).value() {
                        let first = list.first(context);
                        list = list.pop(context);

                        stack.push_front(list.as_value());
                        stack.push_front(first);
                        states.push_front(State::Do);
                        states.push_front(State::Eval);
                    } else {
                        stack.push_front(value);
                    }
                },
                State::Fn => {
                    let args = stack.pop_front().unwrap();
                    let scope = scopes.front().unwrap();
                    let func = Function::constructor(context, *scope, args.downcast::<Object<List>>().unwrap());
                    stack.push_front(func);
                },
                State::If => {
                    let mut expr = stack.pop_front().unwrap();
                    let mut if_statement = stack.pop_front().unwrap();
                    let mut else_statement = stack.pop_front().unwrap();

                    if expr.typ() == context.BooleanType && expr.downcast::<Object<bool>>().unwrap().value() == &true {
                        stack.push_front(if_statement);
                        states.push_front(State::Eval);
                    } else {
                        stack.push_front(else_statement);
                        states.push_front(State::Eval);
                    }
                },
                State::Let => {
                    stack.push_front(context.nil_value.as_value());
                    states.push_front(State::Throw);
                },
                State::Macro => {
                    let args = stack.pop_front().unwrap();
                    let scope = scopes.front().unwrap();
                    let mac = Function::macro_constructor(context, *scope, args.downcast::<Object<List>>().unwrap());
                    stack.push_front(mac);
                },
                State::Quote => {
                    // TODO: remove state?
                },
                State::Type => {
                    let args = stack.pop_front().unwrap();
                    let scope = scopes.front().unwrap();
                    let typ = Type::constructor(context, *scope, args.downcast::<Object<List>>().unwrap());
                    stack.push_front(typ);
                },
                State::Throw => {
                    panic!("throw {:?}", stack.pop_front().unwrap());
                },
            },
            None => break,
        }
    }

    stack.pop_front().unwrap()
}
