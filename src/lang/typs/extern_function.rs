use super::super::super::Ptr;
use super::super::Value;
use super::super::Object;
use super::Function;
use super::List;


pub struct ExternFunction {
    ptr: fn(Ptr<Object<List<Ptr<Value>>>>) -> Ptr<Value>,
}

impl ExternFunction {

    #[inline(always)]
    pub fn new(ptr: fn(Ptr<Object<List<Ptr<Value>>>>) -> Ptr<Value>) -> Self {
        ExternFunction {
            ptr: ptr,
        }
    }
}
    
impl Function for ExternFunction {

    #[inline(always)]
    fn call(&self, args: Ptr<Object<List<Ptr<Value>>>>) -> Ptr<Value> {
        (self.ptr)(args)
    }
}