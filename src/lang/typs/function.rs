use super::super::super::utils::Ptr;
use super::super::value::Value;
use super::super::object::Object;
use super::list::List;


pub trait Function {
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value>;
}

impl Function for fn(args: Ptr<Object<List>>) -> Ptr<Value> {

    #[inline(always)]
    fn call(&self, args: Ptr<Object<List>>) -> Ptr<Value> {
        (self)(args)
    }
}
