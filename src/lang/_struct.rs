use core::fmt;
use core::hash::Hasher;

use ::{Context, Hash, Ptr};

use super::object::Object;
use super::typ::Type;
use super::value::Value;
use super::list::List;
use super::hash_map::HashMap;


pub struct Struct {
    map: Ptr<Object<HashMap>>,
}

impl Struct {

    #[inline]
    pub fn new(context: &Context, typ: Ptr<Object<Type>>, mut args: Ptr<Object<List>>) -> Self {
        let fields = typ.fields.as_ref().expect("can not create Struct from fields value which is None");
        let mut map = HashMap::with_capacity(fields.len());

        for field in fields.iter() {
            map.set_mut(field.as_value(), args.first(context));
            args = args.pop(context);
        }

        Struct {
            map: context.gc.new_object(context.HashMapType, map),
        }
    }

    #[inline]
    pub(crate) fn constructor(context: &Context, typ: Ptr<Object<Type>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        context.gc.new_object(typ, Self::new(context, typ, args)).as_value()
    }
}

impl Ptr<Object<Struct>> {

    #[inline]
    fn from_hash_map(context: &Context, typ: Ptr<Object<Type>>, map: Ptr<Object<HashMap>>) -> Self {
        context.gc.new_object(typ, Struct {
            map: map,
        })
    }

    #[inline(always)]
    pub fn has(&self, context: &Context, key: Ptr<Value>) -> Ptr<Object<bool>> {
        if self.map.contains_key(key) {
            context.true_value
        } else {
            context.false_value
        }
    }

    #[inline]
    pub fn set(&mut self, context: &Context, key: Ptr<Value>, value: Ptr<Value>) -> Self {
        if self.map.contains_key(key) {
            Self::from_hash_map(
                context,
                self.typ(),
                self.map.set(context, key, value),
            )
        } else {
            *self
        }
    }

    #[inline]
    pub fn get(&self, context: &Context, key: Ptr<Value>) -> Ptr<Value> {
        self.map.get(context, key)
    }
}

impl Hash for Struct {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.map.iter() {
            Hash::hash(k, state);
            Hash::hash(v, state);
        }
    }
}

impl PartialEq for Struct {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if self.map.size() == other.map.size() {
            for (ak, av) in self.map.iter() {
                match (&**other.map).get(*ak) {
                    Some(bv) => if av.equals(bv) {
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

impl Eq for Struct {}

impl fmt::Display for Struct {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.map, f)
    }
}

impl fmt::Debug for Struct {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
