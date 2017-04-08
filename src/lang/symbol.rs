use collections::string::String;

use core::fmt;
use core::ops::{Deref, DerefMut};


pub struct Symbol {
    value: String,
}

impl fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Symbol {
    #[inline(always)]
    pub fn new(string: String) -> Self {
        Symbol {
            value: string,
        }
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
