use core::any::Any;

use super::Type;
use super::Object;


pub trait Value: Any + Send + Sync {
    fn typ(&self) -> &Object<Type>;
    fn typ_mut(&mut self) -> &mut Object<Type>;
}

impl_any!(Value);
