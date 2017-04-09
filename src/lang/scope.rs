use collections::string::String;

use collection_traits::*;
use hash_map::HashMap;

use ::Ptr;

use super::object::Object;
use super::value::Value;
use super::symbol::Symbol;


pub struct Scope {
    name: Option<Ptr<Object<Symbol>>>,
    parent: Option<Ptr<Object<Scope>>>,
    mappings: HashMap<String, Ptr<Value>>,
}

impl Scope {

    #[inline(always)]
    pub fn new(name: Option<Ptr<Object<Symbol>>>, parent: Option<Ptr<Object<Scope>>>) -> Self {
        Scope {
            name: name,
            parent: parent,
            mappings: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn name(&self) -> Option<Ptr<Object<Symbol>>> {
        self.name
    }
    #[inline(always)]
    pub fn parent(&self) -> Option<Ptr<Object<Scope>>> {
        self.parent
    }

    #[inline]
    pub fn contains(&self, symbol: &str) -> bool {
        if self.mappings.contains_key(symbol) {
            true
        } else if let Some(ref parent) = self.parent {
            parent.contains(symbol)
        } else {
            false
        }
    }
    #[inline]
    pub fn get(&self, symbol: &str) -> Option<Ptr<Value>> {
        if let Some(value) = self.mappings.get(symbol) {
            Some(value.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get(symbol)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_defined_scope_mut(&self, symbol: &str) -> Option<Ptr<Object<Scope>>> {
        if let Some(ref parent) = self.parent {
            if parent.mappings.contains_key(symbol) {
                Some(parent.clone())
            } else {
                parent.get_defined_scope_mut(symbol)
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn set(&mut self, symbol: &str, value: Ptr<Value>) {
        let string: String = symbol.into();

        if let Some(ref mut scope) = self.get_defined_scope_mut(&string) {
            scope.mappings.insert(string, value);
        } else {
            self.mappings.insert(string, value);
        }
    }
}
