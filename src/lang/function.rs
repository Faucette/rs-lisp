use ::{Context, Ptr};

use super::object::Object;
use super::value::Value;
use super::list::List;
use super::callable::Callable;


pub enum Function {
    Rust(fn(&Context, Ptr<Object<List>>) -> Ptr<Value>),
}

impl Function {

    #[inline(always)]
    pub fn new_rust(fn_ptr: fn(&Context, Ptr<Object<List>>) -> Ptr<Value>) -> Self {
        Function::Rust(fn_ptr)
    }
}

impl Callable for Function {

    #[inline]
    fn call(&self, context: &Context, args: Ptr<Object<List>>) -> Ptr<Value> {
        match self {
            &Function::Rust(ref fn_ptr) => Callable::call(fn_ptr, context, args),
        }
    }
}
