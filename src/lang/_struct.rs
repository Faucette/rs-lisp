use collections::string::{String, ToString};

use core::fmt;

use collection_traits::*;
use hash_map::HashMap;
use vector::Vector;

use ::{Context, Ptr};

use super::object::Object;
use super::typ::Type;
use super::value::Value;
use super::keyword::Keyword;
use super::symbol::Symbol;
use super::list::List;


pub struct Struct {
    map: HashMap<String, Ptr<Value>>,
}

impl Struct {

    #[inline]
    pub fn new(context: &Context, typ: Ptr<Object<Type>>, mut args: Ptr<Object<List>>) -> Self {
        let fields = typ.fields.as_ref().expect("can not create Struct from fields value which is None");
        let mut map = HashMap::with_capacity(fields.len());

        for field in fields.iter() {
            map.insert(field.clone(), args.first(context));
            args = args.pop(context);
        }

        Struct {
            map: map
        }
    }

    #[inline]
    pub(crate) fn constructor(context: &Context, typ: Ptr<Object<Type>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        context.gc.new_object(typ, Self::new(context, typ, args)).as_value()
    }

    #[inline(always)]
    pub fn has(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    #[inline(always)]
    pub fn set(&mut self, key: &str, value: Ptr<Value>) {
        if self.map.contains_key(key) {
            self.map.insert(key.into(), value);
        }
    }

    #[inline(always)]
    pub fn get(&self, key: &str) -> Option<&Ptr<Value>> {
        self.map.get(key)
    }

    #[inline]
    pub fn key_to_string<'a>(context: &Context, key: &Ptr<Value>) -> String {
        if key.typ() == context.SymbolType {
            let symbol = key.downcast::<Object<Symbol>>().unwrap();
            (*symbol.value()).clone()
        } else if key.typ() == context.KeywordType {
            let keyword = key.downcast::<Object<Keyword>>().unwrap();
            (*keyword.value()).clone()
        } else if key.typ() == context.StringType {
            let string = key.downcast::<Object<String>>().unwrap();
            string.value().clone()
        } else {
            key.to_string()
        }
    }
}

impl Ptr<Object<Struct>> {

    #[inline(always)]
    pub fn has(&self, context: &Context, key: Ptr<Value>) -> bool {
        self.map.contains_key(&Struct::key_to_string(context, &key))
    }

    #[inline(always)]
    pub fn set(&mut self, context: &Context, key: Ptr<Value>, value: Ptr<Value>) {
        let k = Struct::key_to_string(context, &key);

        if self.map.contains_key(&k) {
            self.map.insert(k, value);
        }
    }

    #[inline(always)]
    pub fn get(&self, context: &Context, key: Ptr<Value>) -> Ptr<Value> {
        match self.map.get(&Struct::key_to_string(context, &key)) {
            Some(value) => *value,
            None => context.nil_value.as_value(),
        }
    }
}

impl fmt::Display for Struct {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.map)
    }
}

impl fmt::Debug for Struct {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}