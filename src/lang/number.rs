use core::{fmt, mem};
use core::hash::{Hash, Hasher};
use core::convert::From;
use core::ops::*;

use ::{Context, Ptr};

use super::value::Value;
use super::object::Object;
use super::list::List;
use super::scope::Scope;


#[derive(Clone, Copy)]
pub struct Number {
    bits: f64,
}

impl Number {

    #[inline(always)]
    pub fn new(bits: f64) -> Number {
        Number {
            bits: bits,
        }
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, _args: Ptr<Object<List>>) -> Ptr<Value> {
        context.gc.new_object(context.NumberType, Self::new(0_f64)).as_value()
    }
}

macro_rules! impl_from {
    ($($t:ty),*) => (
        $(impl From<$t> for Number {
            #[inline(always)]
            fn from(value: $t) -> Number {
                Number::new(value as f64)
            }
        })*
    );
}

impl_from!(
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
);

impl From<f64> for Number {
    #[inline(always)]
    fn from(value: f64) -> Number {
        Number::new(value)
    }
}


macro_rules! impl_primitive_from {
    ($($t:ty),*) => (
        $(impl From<Number> for $t {
            #[inline(always)]
            fn from(value: Number) -> $t {
                value.bits as $t
            }
        })*
    );
}

impl_primitive_from!(
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
);

impl From<Number> for f64 {
    #[inline(always)]
    fn from(value: Number) -> f64 {
        value.bits
    }
}


macro_rules! impl_binop {
    ($($t:ident $fn:ident $binop:tt),*) => (
        $(impl $t for Number {
            type Output = Number;

            #[inline(always)]
            fn $fn(self, other: Number) -> Self::Output {
                Number::new(self.bits $binop other.bits)
            }
        })*

        $(impl<'a> $t for &'a Number {
            type Output = Number;

            #[inline(always)]
            fn $fn(self, other: &'a Number) -> Self::Output {
                Number::new(self.bits $binop other.bits)
            }
        })*
    );
}

impl_binop!(
    Add add +,
    Sub sub -,
    Mul mul *,
    Div div /,
    Rem rem %
);


impl Hash for Number {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(unsafe {
            mem::transmute::<&f64, &u64>(&self.bits)
        }, state);
    }
}

impl PartialEq for Number {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl Eq for Number {}

impl fmt::Display for Number {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.bits)
    }
}

impl fmt::Debug for Number {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Deref for Number {
    type Target = f64;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}

impl DerefMut for Number {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bits
    }
}
