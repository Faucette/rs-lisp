use core::any::Any;

use super::super::utils::Ptr;
use super::typ::Type;
use super::object::Object;


pub trait Value: Any + Send + Sync {
    fn typ(&self) -> Ptr<Object<Type>>;
}

impl_any!(Value);
