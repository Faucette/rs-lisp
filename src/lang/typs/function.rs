use alloc::boxed::Box;

use super::super::super::utils::Ptr;
use super::super::value::Value;
use super::super::object::Object;
use super::list::List;


pub trait Function {
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value>;
}

impl Function {
    pub fn new(function: fn(Ptr<Object<List>>) -> Ptr<Value>) -> Ptr<Function> {
        unsafe {
            Ptr::from_ptr(Box::into_raw(Box::new(RustFunciton::new(function))))
        }
    }
}


impl Function for fn(args: Ptr<Object<List>>) -> Ptr<Value> {

    #[inline(always)]
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value> {
        (self)(args)
    }
}


struct RustFunciton {
    function: fn(Ptr<Object<List>>) -> Ptr<Value>,
}

impl RustFunciton {

    #[inline(always)]
    fn new(function: fn(Ptr<Object<List>>) -> Ptr<Value>) -> Self {
        RustFunciton {
            function: function,
        }
    }
}

impl Function for RustFunciton {
    #[inline(always)]
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value> {
        (self.function)(args)
    }
}
