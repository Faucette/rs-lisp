use collections::string::String;

use core::{fmt, hash};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

use ::{Context, Ptr};

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::keyword::Keyword;
use super::scope::Scope;


pub struct Symbol {
    value: String,
}

impl Symbol {

    #[inline(always)]
    pub fn new(string: String) -> Self {
        Symbol {
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

        context.gc.new_object(context.SymbolType, Self::new(name)).as_value()
    }
}

impl Hash for Symbol {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.value, state);
    }
}

impl PartialEq for Symbol {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.value, &other.value)
    }
}

impl Eq for Symbol {}

impl fmt::Display for Symbol {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl fmt::Debug for Symbol {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Deref for Symbol {
    type Target = String;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Symbol {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
