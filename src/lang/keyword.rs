use collections::string::String;

use core::fmt;
use core::ops::{Deref, DerefMut};


pub struct Keyword {
    value: String,
}

impl fmt::Debug for Keyword {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Keyword {
    #[inline(always)]
    pub fn new(string: String) -> Self {
        Keyword {
            value: string,
        }
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
