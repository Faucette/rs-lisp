use alloc::boxed::Box;
use collections::string::String;

use collection_traits::*;
use hash_map::HashMap;

use super::super::gc::GcObject;
use super::value::Value;
use super::object::Object;
use super::typ::Type;


pub struct Scope {
    parent: Option<GcObject<Scope>>,
    defines: HashMap<String, Box<Value>>,
}

unsafe impl Send for Scope {}
unsafe impl Sync for Scope {}

impl Scope {

    #[inline(always)]
    pub fn new(parent: Option<GcObject<Scope>>) -> Self {
        Scope {
            parent: parent,
            defines: HashMap::new(),
        }
    }

    #[inline]
    pub fn contains(&self, ident: &str) -> bool {
        if self.defines.contains_key(ident) {
            true
        } else if let Some(ref parent) = self.parent {
            parent.contains(ident)
        } else {
            false
        }
    }

    #[inline]
    pub fn get(&self, ident: &str) -> Option<&Box<Value>> {
        if let Some(ref value) = self.defines.get(ident) {
            Some(value)
        } else if let Some(ref parent) = self.parent {
            parent.get(ident)
        } else {
            None
        }
    }

    #[inline]
    fn get_defined_scope_mut(&mut self, ident: &str) -> Option<&mut GcObject<Scope>> {
        if let Some(ref mut parent) = self.parent {
            if parent.defines.contains_key(ident) {
                Some(parent)
            } else {
                parent.get_defined_scope_mut(ident)
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn define<T: 'static + Sync + Send>(&mut self, ident: String, value: Object<T>) {
        if let Some(ref mut scope) = self.get_defined_scope_mut(&ident) {
            scope.defines.insert(ident, Box::new(value));
            return;
        }
        self.defines.insert(ident, Box::new(value));
    }
}
