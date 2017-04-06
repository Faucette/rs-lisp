use super::super::super::Ptr;
use super::super::Value;
use super::super::Object;
use super::List;


pub trait Function {
    fn call(&self, args: Ptr<Object<List<Ptr<Value>>>>) -> Ptr<Value>;
}