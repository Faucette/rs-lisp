use core::fmt;
use core::hash::Hasher;

use ::{Context, Hash, Ptr};

use super::object::Object;
use super::typ::Type;
use super::value::Value;
use super::list::List;
use super::hash_map::HashMap;


pub struct Interface {
    map: Ptr<Object<HashMap>>,
}

impl Interface {

    #[inline]
    pub fn new(context: &Context, typ: Ptr<Object<Type>>, mut args: Ptr<Object<List>>) -> Self {
        let fields = typ.fields.as_ref().expect("can not create Interface from fields value which is None");
        let mut map = HashMap::with_capacity(fields.len());

        for field in fields.iter() {
            map.set_mut(field.as_value(), args.first(context));
            args = args.pop(context);
        }

        Interface {
            map: context.gc.new_object(context.HashMapType, map),
        }
    }

    #[inline]
    pub(crate) fn constructor(context: &Context, typ: Ptr<Object<Type>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        context.gc.new_object(typ, Self::new(context, typ, args)).as_value()
    }
}

impl Hash for Interface {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.map.iter() {
            Hash::hash(k, state);
            Hash::hash(v, state);
        }
    }
}

impl PartialEq for Interface {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Eq for Interface {}

impl fmt::Display for Interface {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.map, f)
    }
}

impl fmt::Debug for Interface {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
