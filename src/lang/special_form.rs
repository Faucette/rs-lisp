use collections::string::String;

use core::fmt;
use core::ops::{Deref, DerefMut};

use ::{Context, Ptr};

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::scope::Scope;


pub struct SpecialForm {
    value: String,
}

impl SpecialForm {

    #[inline(always)]
    pub fn new(string: String) -> Self {
        SpecialForm {
            value: string,
        }
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        let name: String = {
            let value = args.first(context);

            if value.typ() == context.SpecialFormType {
                let keyword = value.downcast::<Object<SpecialForm>>().unwrap();
                (*keyword.value()).clone()
            } else {
                panic!("invalid value argument should be keyword") // TODO throw runtime exception
            }
        };

        context.gc.new_object(context.SpecialFormType, Self::new(name)).as_value()
    }
}

impl fmt::Display for SpecialForm {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":{}", self.value)
    }
}

impl fmt::Debug for SpecialForm {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Deref for SpecialForm {
    type Target = String;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for SpecialForm {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
