use alloc::boxed::Box;

use ::Ptr;
use ::Context;

use super::value::Value;
use super::object::Object;
use super::list::List;


pub trait Function {
    fn call(&self, &Context, Ptr<Object<List>>) -> Ptr<Value>;
}

impl Function {
    pub fn new(function: fn(&Context, Ptr<Object<List>>) -> Ptr<Value>) -> Ptr<Function> {
        unsafe {
            Ptr::from_ptr(Box::into_raw(Box::new(RustFunciton::new(function))))
        }
    }
}


impl Function for fn(&Context, Ptr<Object<List>>) -> Ptr<Value> {

    #[inline(always)]
    fn call(&self, context: &Context, args: Ptr<Object<List>>) -> Ptr<Value> {
        (self)(context, args)
    }
}


struct RustFunciton {
    function: fn(&Context, Ptr<Object<List>>) -> Ptr<Value>,
}

impl RustFunciton {

    #[inline(always)]
    fn new(function: fn(&Context, Ptr<Object<List>>) -> Ptr<Value>) -> Self {
        RustFunciton {
            function: function,
        }
    }
}

impl Function for RustFunciton {
    #[inline(always)]
    fn call(&self, context: &Context, args: Ptr<Object<List>>) -> Ptr<Value> {
        Function::call(&self.function, context, args)
    }
}
