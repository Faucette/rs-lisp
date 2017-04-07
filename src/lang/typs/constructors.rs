use super::super::super::utils::Ptr;
use super::super::{NIL};
use super::super::{List, Nil};
use super::super::value::Value;
use super::super::object::Object;


#[inline(always)]
pub fn nil(_args: Ptr<Object<List>>) -> Ptr<Value> {
    Object::new(unsafe {NIL}, Nil).as_value()
}
