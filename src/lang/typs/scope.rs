use collection_traits::*;
use hash_map::HashMap;

use super::super::super::utils::Ptr;
use super::super::object::Object;
use super::super::value::Value;
use super::symbol::Symbol;


pub struct Scope {
    name: Option<Ptr<Object<Symbol>>>,
    mappings: HashMap<String, Ptr<Value>>,
}

impl Scope {

    #[inline(always)]
    pub fn new(name: Option<Ptr<Object<Symbol>>>) -> Self {
        Scope {
            name: name,
            mappings: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn name(&self) -> Option<Ptr<Object<Symbol>>> {
        self.name
    }

    #[inline(always)]
    pub fn set(&mut self, symbol: Ptr<Object<Symbol>>, value: Ptr<Value>) {
        self.mappings.insert((**symbol).clone(), value);
    }

    #[inline(always)]
    pub fn get(&mut self, symbol: Ptr<Object<Symbol>>) -> Option<&Ptr<Value>> {
        self.get_by_str(&**symbol)
    }

    #[inline(always)]
    pub fn get_by_str(&mut self, symbol: &str) -> Option<&Ptr<Value>> {
        self.mappings.get(symbol)
    }

    #[inline(always)]
    pub fn remove(&mut self, symbol: Ptr<Object<Symbol>>) {
        self.mappings.remove(&**symbol);
    }
}
