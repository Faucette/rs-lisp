use core::fmt;

use ::{Context, Ptr, eval};

use super::object::Object;
use super::value::Value;
use super::typ::Type;
use super::list::List;
use super::scope::Scope;
use super::_struct::Struct;
use super::symbol::Symbol;
use super::keyword::Keyword;
use super::callable::Callable;


pub enum Function {
    Constructor(Ptr<Object<Type>>),
    Rust(fn(&Context, Ptr<Object<Scope>>, Ptr<Object<List>>) -> Ptr<Value>),
    Internal(Ptr<Object<Scope>>, Option<Ptr<Object<Symbol>>>, Ptr<Object<List>>, Ptr<Value>),
}

impl Function {

    #[inline(always)]
    pub fn new_rust(fn_ptr: fn(&Context, Ptr<Object<Scope>>, Ptr<Object<List>>) -> Ptr<Value>) -> Self {
        Function::Rust(fn_ptr)
    }

    #[inline(always)]
    pub fn new_constructor(typ: Ptr<Object<Type>>) -> Self {
        Function::Constructor(typ)
    }

    #[inline(always)]
    pub fn new_internal(
        scope: Ptr<Object<Scope>>,
        name: Option<Ptr<Object<Symbol>>>,
        args: Ptr<Object<List>>,
        body: Ptr<Value>
    ) -> Self {
        Function::Internal(scope, name, args, body)
    }

    #[inline(always)]
    pub fn constructor(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        Self::generic_constructor(context.FunctionType, context, scope, args)
    }
    #[inline(always)]
    pub fn special_form_constructor(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        Self::generic_constructor(context.SpecialFormType, context, scope, args)
    }
    #[inline(always)]
    pub fn macro_constructor(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        Self::generic_constructor(context.MacroType, context, scope, args)
    }

    #[inline]
    pub fn generic_constructor(typ: Ptr<Object<Type>>, context: &Context, scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
        let name: Option<Ptr<Object<Symbol>>> = {
            let value = args.first(context);

            if value.typ() == context.KeywordType {
                args = args.pop(context);
                let keyword = value.downcast::<Object<Keyword>>().unwrap();
                let string = (*keyword.value()).clone();
                Some(context.gc.new_object(context.SymbolType, Symbol::new(string)))
            } else if value.typ() == context.SymbolType {
                args = args.pop(context);
                value.downcast::<Object<Symbol>>()
            } else {
                None
            }
        };

        let arg_symbols = {
            let value = args.first(context);

            if value.typ() == context.ListType {
                args = args.pop(context);
                value.downcast::<Object<List>>().unwrap()
            } else {
                panic!("invalid arguments argument should be list")
            }
        };

        let body = args.first(context);

        context.gc.new_object(typ,
            Function::new_internal(scope, name, arg_symbols, body)).as_value()
    }
}

impl Callable for Ptr<Object<Function>> {

    #[inline]
    fn call(&self, context: &Context, scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
        match &***self {
            &Function::Constructor(typ) => Struct::constructor(context, typ, args),
            &Function::Rust(ref fn_ptr) => Callable::call(fn_ptr, context, scope, args),
            &Function::Internal(scope, name, mut arg_symbols, body) => {
                let mut function_scope =
                    context.gc.new_object(context.ScopeType, Scope::new(None, Some(scope)));

                if let Some(name) = name {
                    function_scope.set(name.value().clone(), self.as_value());
                }

                while !arg_symbols.is_empty(context).value() {
                    let symbol = arg_symbols.first(context);
                    let value = args.first(context);

                    args = args.pop(context);
                    arg_symbols = arg_symbols.pop(context);

                    let symbol = symbol.downcast::<Object<Symbol>>().unwrap();
                    function_scope.set(symbol.value().clone(), value);
                }

                eval(context, function_scope, body)
            },
        }
    }
}

impl fmt::Display for Function {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Constructor(_) => write!(f, ":constructor_function"),
            &Function::Rust(_) => write!(f, ":native_function"),
            &Function::Internal(_, name, args, body) => {
                if let Some(n) = name {
                    write!(f, "(fn {} {} {})", n, args, body)
                } else {
                    write!(f, "(fn {} {})", args, body)
                }
            }
        }
    }
}

impl fmt::Debug for Function {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
