use super::super::super::utils::Ptr;
use super::super::value::Value;
use super::super::object::Object;
use super::builtins::NIL;
use super::list::List;
use super::nil::Nil;


#[inline(always)]
pub fn nil(_args: Ptr<Object<List>>) -> Ptr<Value> {
    Object::new(unsafe {NIL}, Nil).as_value()
}
