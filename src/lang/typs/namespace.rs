use collection_traits::*;
use hash_map::HashMap;

use super::super::super::Ptr;
use super::super::Object;
use super::super::Value;
use super::Symbol;


pub struct Namespace {
    name: Ptr<Object<Symbol>>,
    mappings: HashMap<String, Ptr<Value>>,
}

impl Namespace {

    #[inline(always)]
    pub fn new(name: Ptr<Object<Symbol>>) -> Self {
        Namespace {
            name: name,
            mappings: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn name(&self) -> Ptr<Object<Symbol>> {
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
