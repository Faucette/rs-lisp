use collections::string::String;

use core::fmt;
use core::ops::{Deref, DerefMut};

use super::super::super::utils::Ptr;
use super::super::object::Object;
use super::typs::{BOOLEAN, KEYWORD, SYMBOL, NIL};


#[inline(always)]
pub fn Boolean_new(value: bool) -> Ptr<Object<bool>> {
    Object::new(unsafe {BOOLEAN}, value)
}


#[derive(Debug, PartialEq)]
pub struct Nil;


impl Nil {
    #[inline(always)]
    pub fn new() -> Ptr<Object<Nil>> {
        Object::new(unsafe {NIL}, Nil)
    }
}


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
    pub fn new(string: String) -> Ptr<Object<Symbol>> {
        Object::new(unsafe {SYMBOL}, Symbol {
            value: string,
        })
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


pub struct Keyword {
    value: String,
}

impl fmt::Debug for Keyword {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":{}", self.value)
    }
}

impl Keyword {
    #[inline(always)]
    pub fn new(string: String) -> Ptr<Object<Keyword>> {
        Object::new(unsafe {KEYWORD}, Keyword {
            value: string,
        })
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
