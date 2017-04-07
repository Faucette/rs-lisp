use super::super::super::Ptr;
use super::super::NIL;
use super::super::List;
use super::super::{Value, Object};


pub struct Nil;


#[inline(always)]
pub fn nil(_args: Ptr<Object<List>>) -> Ptr<Value> {
    Object::new(unsafe {NIL}, Nil).as_value()
}
