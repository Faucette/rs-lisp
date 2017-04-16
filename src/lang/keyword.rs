use collections::string::{ToString, String};

use core::fmt;
use core::hash::Hasher;
use core::ops::{Deref, DerefMut};

use ::{Context, Hash, Ptr};

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::scope::Scope;
use super::symbol::Symbol;


pub struct Keyword {
    value: String,
}

impl Keyword {

    #[inline(always)]
    pub fn new(string: String) -> Self {
        Keyword {
            value: string,
        }
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        let name: String = {
            let value = args.first(context);

            if value.typ() == context.KeywordType {
                let keyword = value.downcast::<Object<Keyword>>().unwrap();
                (*keyword.value()).clone()
            } else {
                panic!("invalid value argument should be keyword") // TODO throw runtime exception
            }
        };

        context.gc.new_object(context.KeywordType, Self::new(name)).as_value()
    }

    #[inline]
    pub fn to_keyword(context: &Context, value: Ptr<Value>) -> Ptr<Object<Keyword>> {
        if value.typ() == context.KeywordType {
            value.downcast::<Object<Keyword>>().unwrap()
        } else if value.typ() == context.SymbolType {
            let symbol = value.downcast::<Object<Symbol>>().unwrap();
            let string = (*symbol.value()).clone();
            context.gc.new_object(context.KeywordType, Self::new(string))
        } else if value.typ() == context.StringType {
            let string = value.downcast::<Object<String>>().unwrap();
            let string = (*string.value()).clone();
            context.gc.new_object(context.KeywordType, Self::new(string))
        } else {
            let string = value.to_string();
            context.gc.new_object(context.KeywordType, Self::new(string))
        }
    }
}

impl Hash for Keyword {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.value, state);
    }
}

impl PartialEq for Keyword {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.value, &other.value)
    }
}

impl Eq for Keyword {}

impl fmt::Display for Keyword {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":{}", self.value)
    }
}

impl fmt::Debug for Keyword {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Deref for Keyword {
    type Target = String;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Keyword {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
