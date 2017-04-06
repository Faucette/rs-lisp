use core::any::Any;

use super::super::Ptr;
use super::{Object, Type};


pub trait Value: Any + Send + Sync {
    fn typ(&self) -> Ptr<Object<Type>>;
}

impl_any!(Value);
