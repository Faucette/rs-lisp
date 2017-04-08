use core::fmt;


#[derive(PartialEq)]
pub struct Nil;

impl Nil {

    #[inline(always)]
    pub fn new() -> Nil { Nil }
}



impl fmt::Debug for Nil {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nil")
    }
}
