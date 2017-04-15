use core::fmt;
use core::hash::{Hasher, BuildHasher};

use hash_map;
use collection_traits::*;

use ::{Context, LHash, Ptr};
use ::lang::{Object, Value, List, Scope};


pub struct HashMap {
    map: hash_map::HashMap<u64, Ptr<Value>>,
}

impl HashMap {

    #[inline(always)]
    pub fn new() -> Self {
        HashMap {
            map: hash_map::HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        let mut hash_map = Self::new();

        let mut it = args.iter();
        loop {
            if let Some(key) = it.next() {
                match it.next() {
                    Some(value) => hash_map.set_mut(key, value),
                    None => hash_map.set_mut(key, context.nil_value.as_value()),
                }
            } else {
                break;
            }
        }

        context.gc.new_object(context.HashMapType, hash_map).as_value()
    }

    #[inline]
    pub fn key_hash(&self, key: Ptr<Value>) -> u64 {
        let mut hasher = self.map.hasher().build_hasher();
        key.typ().hash(&mut hasher);
        hasher.finish()
    }

    #[inline]
    pub fn contains_key(&self, key: Ptr<Value>) -> bool {
        self.map.contains_key(&self.key_hash(key))
    }

    #[inline(always)]
    pub fn get(&self, key: Ptr<Value>) -> Option<Ptr<Value>> {
        match self.map.get(&self.key_hash(key)) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    #[inline]
    pub fn set_mut(&mut self, key: Ptr<Value>, value: Ptr<Value>) {
        let hash = self.key_hash(key);
        self.map.insert(hash, value);
    }

    #[inline(always)]
    pub fn iter(&self) -> hash_map::Iter<u64, Ptr<Value>> {
        self.map.iter()
    }
}

impl Ptr<Object<HashMap>> {

    #[inline]
    pub fn clone(&self, context: &Context) -> Self {
        context.gc.new_object(context.HashMapType, HashMap {
            map: self.map.clone(),
        })
    }

    #[inline]
    pub fn set(&self, context: &Context, key: Ptr<Value>, value: Ptr<Value>) -> Self {
        let mut new_map = self.clone(context);
        new_map.map.insert(self.key_hash(key), value);
        new_map
    }

    #[inline(always)]
    pub fn get(&self, context: &Context, key: Ptr<Value>) -> Ptr<Value> {
        match self.map.get(&self.key_hash(key)) {
            Some(value) => *value,
            None => context.nil_value.as_value(),
        }
    }

    #[inline]
    pub fn remove(&self, context: &Context, key: Ptr<Value>) -> Self {
        if self.contains_key(key) {
            let mut new_map = self.clone(context);
            new_map.map.remove(&self.key_hash(key));
            new_map
        } else {
            *self
        }
    }
}

impl LHash for HashMap {

    #[inline(always)]
    fn hash(&self, state: &mut hash_map::DefaultHasher) {
        ((&self.map) as *const _ as usize).hash(state);
    }
}

impl fmt::Display for HashMap {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        let mut it = self.iter();
        while let Some((key, value)) = it.next() {
            let (size, _) = it.size_hint();

            if size > 0 {
                write!(f, "{:?} {:?} ", key, value)?;
            } else {
                write!(f, "{:?} {:?}", key, value)?;
            }
        }
        write!(f, "}}")
    }
}

impl fmt::Debug for HashMap {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
