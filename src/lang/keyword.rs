use collections::string::String;

use core::fmt;
use core::ops::{Deref, DerefMut};

use ::{Context, LHash, Ptr};

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::scope::Scope;


#[derive(Hash)]
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
}

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
