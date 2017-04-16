use core::fmt;
use core::hash::{Hash, Hasher};

use hash_map;
use collection_traits::*;

use ::{Context, Ptr};
use ::lang::{Object, Value, List, Scope};


pub struct HashMap {
    //map: hash_map::HashMap<Ptr<Value>, Ptr<Value>>,
    map: hash_map::HashMap<Ptr<Value>, Ptr<Value>>,
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
    pub fn contains_key(&self, key: Ptr<Value>) -> bool {
        self.map.contains_key(&key)
    }

    #[inline(always)]
    pub fn get(&self, key: Ptr<Value>) -> Option<Ptr<Value>> {
        match self.map.get(&key) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    #[inline]
    pub fn set_mut(&mut self, key: Ptr<Value>, value: Ptr<Value>) {
        let hash = key;
        self.map.insert(hash, value);
    }

    #[inline(always)]
    pub fn iter(&self) -> hash_map::Iter<Ptr<Value>, Ptr<Value>> {
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
        new_map.map.insert(key, value);
        new_map
    }

    #[inline(always)]
    pub fn get(&self, context: &Context, key: Ptr<Value>) -> Ptr<Value> {
        match self.map.get(&key) {
            Some(value) => *value,
            None => context.nil_value.as_value(),
        }
    }

    #[inline]
    pub fn remove(&self, context: &Context, key: Ptr<Value>) -> Self {
        if self.contains_key(key) {
            let mut new_map = self.clone(context);
            new_map.map.remove(&key);
            new_map
        } else {
            *self
        }
    }
}

impl Hash for HashMap {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.iter() {
            Hash::hash(k, state);
            Hash::hash(v, state);
        }
    }
}

impl PartialEq for HashMap {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if self.map.len() == other.map.len() {
            for (ak, av) in self.map.iter() {
                match other.map.get(ak) {
                    Some(bv) => if av.equals(*bv) {
                        return false;
                    },
                    None => {
                        return false;
                    },
                }
            }
            true
        } else {
            false
        }
    }
}

impl Eq for HashMap {}

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
