use collection_traits::*;
use hash_map::HashMap;

use super::super::utils::Ptr;
use super::symbol::Symbol;
use super::object::Object;
use super::value::Value;


pub struct Namespace {
    name: Ptr<Object<Symbol>>,
    mappings: HashMap<Ptr<Object<Symbol>>, Ptr<Value>>,
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
    pub fn set(&mut self, symbol: Ptr<Object<Symbol>>, value: Ptr<Value>) {
        self.mappings.insert(symbol, value);
    }

    #[inline(always)]
    pub fn remove(&mut self, symbol: Ptr<Object<Symbol>>) {
        self.mappings.remove(&symbol);
    }
}
