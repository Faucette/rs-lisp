use collection_traits::*;
use linked_list::LinkedList;

use ::{Context, Ptr};
use ::lang::{Value, Object, List, Function, Symbol, Struct, Scope, Type};


#[derive(Debug)]
pub enum State {
    Eval,
    EvalList,
    EvalArguments,
    EvalFunction,
    EvalMacro,
    PopScope,
    Def,
    Do,
    Fn,
    If,
    Let,
    Macro,
    Quote,
    Type,
    Throw,
}


#[inline]
pub fn eval(context: &Context, scope: Ptr<Object<Scope>>, value: Ptr<Value>) -> Ptr<Value> {
    let mut scope_stack: LinkedList<Ptr<Object<Scope>>> = LinkedList::new();
    let mut stack: LinkedList<Ptr<Value>> = LinkedList::new();
    let mut state_stack: LinkedList<State> = LinkedList::new();

    scope_stack.push_front(scope);
    stack.push_front(value);
    state_stack.push_front(State::Eval);

    loop {

        match state_stack.pop_front() {

            Some(state) => match state {

                State::Eval => {
                    let value = stack.pop_front().unwrap();
                    let typ = value.typ();

                    if typ == context.SymbolType {
                        if let Some(value) = scope_stack.front().unwrap().get(value) {
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

                                        state_stack.push_front(State::Def);
                                        state_stack.push_front(State::Eval);
                                    },
                                    "do" => {
                                        let first = list.first(context);
                                        list = list.pop(context);

                                        stack.push_front(list.as_value());
                                        stack.push_front(first);
                                        state_stack.push_front(State::Do);
                                        state_stack.push_front(State::Eval);
                                    },
                                    "fn" => {
                                        stack.push_front(list.as_value());
                                        state_stack.push_front(State::Fn);
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

                                        state_stack.push_front(State::If);
                                        state_stack.push_front(State::Eval);
                                    },
                                    "let" => {
                                        let statements = list.first(context);
                                        list = list.pop(context);

                                        match statements.downcast::<Object<List>>() {
                                            Some(mut statements) => {
                                                let block = list.first(context);
                                                //list = list.pop(context);

                                                let mut names = context.gc.new_object(context.ListType, List::new(context));
                                                let mut values = context.gc.new_object(context.ListType, List::new(context));

                                                while !statements.is_empty(context).value() {

                                                    names.push_back_mut(context, statements.first(context));
                                                    statements = statements.pop(context);

                                                    values.push_back_mut(context, statements.first(context));
                                                    statements = statements.pop(context);
                                                }

                                                let eval_args = context.gc.new_object(context.ListType, List::new(context));
                                                let first = values.first(context);
                                                values = values.pop(context);

                                                stack.push_front(block);
                                                stack.push_front(names.as_value());
                                                stack.push_front(eval_args.as_value());
                                                stack.push_front(values.as_value());
                                                stack.push_front(first);

                                                state_stack.push_front(State::Let);
                                                state_stack.push_front(State::EvalArguments);
                                                state_stack.push_front(State::Eval);
                                            },
                                            None => {
                                                state_stack.push_front(State::Throw);
                                                stack.push_front(context.gc.new_object(context.StringType,
                                                    format!("invalid argument in let expected list found {:?}", statements)).as_value());
                                            }
                                        }
                                    },
                                    "macro" => {
                                        stack.push_front(list.as_value());
                                        state_stack.push_front(State::Macro);
                                    },
                                    "quote" => {
                                        stack.push_front(list.first(context));
                                        state_stack.push_front(State::Quote);
                                    },
                                    "type" => {
                                        let name = list.first(context);
                                        list = list.pop(context);

                                        let fields = list.first(context);
                                        list = list.pop(context);

                                        let supr = list.first(context);
                                        //list = list.pop(context);

                                        stack.push_front(name);
                                        stack.push_front(fields);
                                        stack.push_front(supr);

                                        state_stack.push_front(State::Type);
                                        state_stack.push_front(State::Eval);
                                    },
                                    "throw" => {
                                        stack.push_front(list.as_value());
                                        state_stack.push_front(State::Throw);
                                    },
                                    _ => {
                                        stack.push_front(list.as_value());
                                        stack.push_front(first);
                                        stack.push_front(first);
                                        state_stack.push_front(State::EvalList);
                                        state_stack.push_front(State::Eval);
                                    }
                                }
                            },
                            None => {
                                state_stack.push_front(State::Throw);
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
                        let mut args = args.downcast::<Object<List>>().unwrap();

                        let first = args.first(context);
                        args = args.pop(context);

                        let eval_args = context.gc.new_object(context.ListType, List::new(context));

                        stack.push_front(callable);
                        stack.push_front(eval_args.as_value());
                        stack.push_front(args.as_value());
                        stack.push_front(first);
                        state_stack.push_front(State::EvalFunction);
                        state_stack.push_front(State::EvalArguments);
                        state_stack.push_front(State::Eval);
                    } else if callable_typ == context.TypeType {
                        let typ = callable.downcast::<Object<Type>>().unwrap();

                        if typ.is_abstract() {
                            stack.push_front(context.gc.new_object(context.StringType,
                                format!("can not call abstract type {:?} as constructor function", callable)).as_value());
                            state_stack.push_front(State::Throw);
                        } else if typ.is_bits() {
                            stack.push_front(context.gc.new_object(context.StringType,
                                format!("creating bit types from front end not supported yet")).as_value());
                            state_stack.push_front(State::Throw);
                        } else {
                            match typ.constructor {
                                Some(constructor) => {
                                    let mut args = args.downcast::<Object<List>>().unwrap();
                                    let first = args.first(context);
                                    args = args.pop(context);

                                    let eval_args = context.gc.new_object(context.ListType, List::new(context));

                                    stack.push_front(constructor.as_value());
                                    stack.push_front(eval_args.as_value());
                                    stack.push_front(args.as_value());
                                    stack.push_front(first);
                                    state_stack.push_front(State::EvalFunction);
                                    state_stack.push_front(State::EvalArguments);
                                    state_stack.push_front(State::Eval);
                                },
                                None => {
                                    stack.push_front(context.gc.new_object(context.StringType,
                                        format!("invalid type no constructor for {:?}", callable)).as_value());
                                    state_stack.push_front(State::Throw);
                                },
                            }
                        }
                    } else if callable_typ == context.MacroType {
                        let args = args.downcast::<Object<List>>().unwrap();

                        stack.push_front(callable);
                        stack.push_front(args.as_value());
                        state_stack.push_front(State::EvalMacro);
                    } else {
                        stack.push_front(context.gc.new_object(context.StringType,
                            format!("can not call {:?}, which is {:?}, as function", symbol, callable)).as_value());
                        state_stack.push_front(State::Throw);
                    }
                },

                State::EvalArguments => {
                    let eval_arg = stack.pop_front().unwrap();
                    let mut args = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let mut eval_args = stack.front_mut().unwrap().downcast::<Object<List>>().unwrap();

                    eval_args.push_back_mut(context, eval_arg);

                    if !args.is_empty(context).value() {
                        let first = args.first(context);
                        args = args.pop(context);

                        stack.push_front(args.as_value());
                        stack.push_front(first);
                        state_stack.push_front(State::EvalArguments);
                        state_stack.push_front(State::Eval);
                    }
                },

                State::EvalMacro => {
                    let args = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let callable = stack.pop_front().unwrap().downcast::<Object<Function>>().unwrap();
                    let scope = callable.get_scope(context, *scope_stack.front().unwrap(), args);

                    match &**callable {
                        &Function::Internal(_, _, _, body) => {
                            scope_stack.push_front(scope);
                            stack.push_front(body);
                            state_stack.push_front(State::Eval);
                            state_stack.push_front(State::PopScope);
                            state_stack.push_front(State::Eval);
                        },
                        &Function::Constructor(typ) => {
                            stack.push_front(Struct::constructor(context, typ, args));
                            state_stack.push_front(State::Eval);
                        },
                        &Function::Rust(ref fn_ptr) => {
                            stack.push_front((fn_ptr)(context, scope, args));
                            state_stack.push_front(State::Eval);
                        },
                    }
                },

                State::EvalFunction => {
                    let args = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let callable = stack.pop_front().unwrap().downcast::<Object<Function>>().unwrap();
                    let scope = callable.get_scope(context, *scope_stack.front().unwrap(), args);

                    match &**callable {
                        &Function::Internal(_, _, _, body) => {
                            scope_stack.push_front(scope);
                            stack.push_front(body);
                            state_stack.push_front(State::PopScope);
                            state_stack.push_front(State::Eval);
                        },
                        &Function::Constructor(typ) => {
                            stack.push_front(Struct::constructor(context, typ, args));
                        },
                        &Function::Rust(ref fn_ptr) => {
                            stack.push_front((fn_ptr)(context, scope, args));
                        },
                    }
                },
                State::PopScope => {
                    scope_stack.pop_front();
                }

                State::Def => {
                    let value = stack.pop_front().unwrap();
                    let symbol = stack.pop_front().unwrap();

                    let scope = scope_stack.front_mut().unwrap();

                    scope.set(context, symbol, value);

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
                        state_stack.push_front(State::Do);
                        state_stack.push_front(State::Eval);
                    } else {
                        stack.push_front(value);
                    }
                },
                State::Fn => {
                    let args = stack.pop_front().unwrap();
                    let scope = scope_stack.front().unwrap();
                    let func = Function::constructor(context, *scope, args.downcast::<Object<List>>().unwrap());
                    stack.push_front(func);
                },
                State::If => {
                    let expr = stack.pop_front().unwrap();
                    let if_statement = stack.pop_front().unwrap();
                    let else_statement = stack.pop_front().unwrap();

                    if expr.typ() == context.BooleanType && expr.downcast::<Object<bool>>().unwrap().value() == &true {
                        stack.push_front(if_statement);
                        state_stack.push_front(State::Eval);
                    } else {
                        stack.push_front(else_statement);
                        state_stack.push_front(State::Eval);
                    }
                },
                State::Let => {
                    let mut values = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let mut names = stack.pop_front().unwrap().downcast::<Object<List>>().unwrap();
                    let block = stack.pop_front().unwrap();

                    let mut throw = false;
                    let mut throw_value = context.nil_value.as_value();

                    let let_scope = {
                        let scope = scope_stack.front().unwrap();
                        context.gc.new_object(context.ScopeType, Scope::new(context, None, Some(*scope)))
                    };

                    while !names.is_empty(context).value() {
                        let symbol = names.first(context);
                        names = names.pop(context);

                        let value = values.first(context);
                        values = values.pop(context);

                        match symbol.downcast::<Object<Symbol>>() {
                            Some(_) => let_scope.set(context, symbol, value),
                            None => {
                                throw = true;
                                throw_value = value;
                                break;
                            }
                        }
                    }

                    if throw {
                        stack.push_front(context.gc.new_object(context.StringType,
                            format!("invalid symbol in let statments expexted symbol found {:?}",
                                throw_value)).as_value());
                        state_stack.push_front(State::Throw);
                    } else {
                        scope_stack.push_front(let_scope);
                        stack.push_front(block);
                        state_stack.push_front(State::PopScope);
                        state_stack.push_front(State::Eval);
                    }
                },
                State::Macro => {
                    let args = stack.pop_front().unwrap();
                    let scope = scope_stack.front().unwrap();
                    let mac = Function::macro_constructor(context, *scope, args.downcast::<Object<List>>().unwrap());
                    stack.push_front(mac);
                },
                State::Quote => {
                    // TODO: remove state?
                },
                State::Type => {
                    let supr = stack.pop_front().unwrap();
                    let fields = stack.pop_front().unwrap();
                    let name = stack.pop_front().unwrap();

                    let typ = Type::new(context, name, fields, supr);
                    stack.push_front(typ.as_value());
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
