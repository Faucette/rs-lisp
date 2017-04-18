use collections::vec::Vec;

use core::sync::atomic::{AtomicPtr, Ordering};

use core::{fmt, ptr};
use core::hash::Hasher;

use ::{Context, Hash, Ptr};

use super::object::Object;
use super::value::Value;
use super::typ::Type;
use super::list::List;
use super::scope::Scope;
use super::symbol::Symbol;
use super::function::Function;


struct MultiFunctionValue {
    types: Vec<Ptr<Object<Type>>>,
    names: Vec<Ptr<Object<Symbol>>>,
    function: Ptr<Object<Function>>,
}

impl MultiFunctionValue {

    #[inline(always)]
    pub fn new(
        types: Vec<Ptr<Object<Type>>>,
        names: Vec<Ptr<Object<Symbol>>>,
        function: Ptr<Object<Function>>
    ) -> Self {
        MultiFunctionValue {
            types: types,
            names: names,
            function: function,
        }
    }
}


pub struct MultiFunction {
    name: Ptr<Object<Symbol>>,
    list: AtomicPtr<Object<List>>,
}

impl MultiFunction {

    #[inline(always)]
    pub fn new(context: &Context, name: Ptr<Object<Symbol>>) -> Self {
        Self::from_list(
            context,
            name,
            context.gc.new_object(context.ListType, List::new(context))
        )
    }

    #[inline(always)]
    pub fn from_list(context: &Context, name: Ptr<Object<Symbol>>, list: Ptr<Object<List>>) -> Self {
        MultiFunction {
            name: name,
            list: unsafe {
                AtomicPtr::new(list.as_ptr())
            },
        }
    }

    #[inline(always)]
    pub fn add(
        &self,
        context: &Context,
        scope: Ptr<Object<Scope>>,
        mut types: Ptr<Object<List>>,
        mut names: Ptr<Object<List>>,
        body: Ptr<Value>
    ) {
        let mut fn_types = Vec::with_capacity(*types.size().value());
        let mut fn_names = Vec::with_capacity(*types.size().value());

        while !types.is_empty(context).value() {
            fn_types.push(types.first(context));
            fn_names.push(names.first(context));
            types = types.pop(context);
            names = names.pop(context);
        }

        let function = context.gc.new_object(context.FunctionType,
            Function::new_internal(scope, Some(self.name), names, body));

        let new_list = self.list().push(context, function.as_value());
        self.list_swap(new_list);
    }

    #[inline(always)]
    fn list(&self) -> Ptr<Object<List>> {
        unsafe {
            Ptr::from_ptr(self.list.load(Ordering::Relaxed))
        }
    }
    #[inline(always)]
    fn list_swap(&self, list: Ptr<Object<List>>) {
        self.list.store(unsafe {list.as_ptr()}, Ordering::Relaxed);
    }
}

impl Hash for MultiFunction {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self.name.value(), state);
        Hash::hash(&self.list, state);
    }
}

impl PartialEq for MultiFunction {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.list(), &other.list())
    }
}

impl Eq for MultiFunction {}

impl fmt::Display for MultiFunction {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fn)")
    }
}

impl fmt::Debug for MultiFunction {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
