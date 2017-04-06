use super::super::super::Ptr;
use super::super::Value;
use super::super::Object;
use super::List;


pub trait Function {
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value>;
}

impl Function for fn(args: Ptr<Object<List>>) -> Ptr<Value> {

    #[inline(always)]
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value> {
        (self)(args)
    }
}
