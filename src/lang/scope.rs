use core::sync::atomic::{AtomicPtr, Ordering};
use core::fmt;
use core::hash::{Hasher};

use hash_map::DefaultHasher;

use ::{Context, LHash, Ptr};

use super::object::Object;
use super::value::Value;
use super::symbol::Symbol;
use super::hash_map::HashMap;


pub struct Scope {
    pub(crate) name: Option<Ptr<Object<Symbol>>>,
    pub(crate) parent: Option<Ptr<Object<Scope>>>,
    pub(crate) mappings: AtomicPtr<Object<HashMap>>,
}

impl LHash for Scope {

    #[inline(always)]
    fn hash(&self, state: &mut DefaultHasher) {
        ((&self) as *const _ as usize).hash(state);
    }
}

impl Scope {

    #[inline(always)]
    pub fn new(context: &Context, name: Option<Ptr<Object<Symbol>>>, parent: Option<Ptr<Object<Scope>>>) -> Self {
        Scope {
            name: name,
            parent: parent,
            mappings: unsafe {
                AtomicPtr::new(context.gc.new_object(context.HashMapType, HashMap::new()).as_ptr())
            },
        }
    }

    #[inline(always)]
    pub fn from_mappings(name: Option<Ptr<Object<Symbol>>>, parent: Option<Ptr<Object<Scope>>>, mappings: Ptr<Object<HashMap>>) -> Self {
        Scope {
            name: name,
            parent: parent,
            mappings: unsafe {
                AtomicPtr::new(mappings.as_ptr())
            },
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

    #[inline(always)]
    fn mappings(&self) -> &Object<HashMap> {
        unsafe {
            &*(self.mappings.load(Ordering::Relaxed) as *const _)
        }
    }
    #[inline(always)]
    fn mappings_mut(&self) -> &mut Object<HashMap> {
        unsafe {
            &mut *(self.mappings.load(Ordering::Relaxed) as *mut _)
        }
    }
    #[inline(always)]
    fn mappings_ptr(&self) -> Ptr<Object<HashMap>> {
        unsafe {
            Ptr::from_ptr(self.mappings.load(Ordering::Relaxed))
        }
    }

    #[inline(always)]
    fn store(&self, hash_map: Ptr<Object<HashMap>>) {
        self.mappings.store(unsafe {hash_map.as_ptr()}, Ordering::Relaxed);
    }

    #[inline]
    pub fn contains(&self, symbol: Ptr<Value>) -> bool {
        if self.mappings().contains_key(symbol) {
            true
        } else if let Some(ref parent) = self.parent {
            parent.contains(symbol)
        } else {
            false
        }
    }
    #[inline]
    pub fn get(&self, symbol: Ptr<Value>) -> Option<Ptr<Value>> {
        if let Some(value) = self.mappings().get(symbol) {
            Some(value.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get(symbol)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_defined_scope_mut(&self, symbol: Ptr<Value>) -> Option<Ptr<Object<Scope>>> {
        if let Some(ref parent) = self.parent {
            if parent.mappings().contains_key(symbol) {
                Some(parent.clone())
            } else {
                parent.get_defined_scope_mut(symbol)
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn set(&self, context: &Context, symbol: Ptr<Value>, value: Ptr<Value>) {
        if let Some(ref mut scope) = self.get_defined_scope_mut(symbol) {
            scope.store(scope.mappings_ptr().set(context, symbol, value));
        } else {
            self.store(self.mappings_ptr().set(context, symbol, value));
        }
    }

    #[inline]
    pub(crate) fn set_mut(&mut self, symbol: Ptr<Value>, value: Ptr<Value>) {
        if let Some(ref mut scope) = self.get_defined_scope_mut(symbol) {
            scope.mappings_mut().set_mut(symbol, value);
        } else {
            self.mappings_mut().set_mut(symbol, value);
        }
    }
}

impl fmt::Display for Scope {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.name {
            write!(f, "(Scope {:?} {:?})", name, self.mappings())
        } else {
            write!(f, "(Scope {:?})", self.mappings())
        }
    }
}

impl fmt::Debug for Scope {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
