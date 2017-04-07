use core::mem;

use super::super::super::Ptr;
use super::super::typ::{Type, TypeBuilder};
use super::super::value::Value;
use super::super::object::Object;
use super::constructors;
use super::list::List;


pub static mut ANY: Ptr<Object<Type>> = Ptr::null();
pub static mut TYP: Ptr<Object<Type>> = Ptr::null();

pub static mut NIL: Ptr<Object<Type>> = Ptr::null();

pub static mut READER: Ptr<Object<Type>> = Ptr::null();

pub static mut LIST: Ptr<Object<Type>> = Ptr::null();
pub static mut SYMBOL: Ptr<Object<Type>> = Ptr::null();

pub static mut NUMBER: Ptr<Object<Type>> = Ptr::null();
pub static mut REAL: Ptr<Object<Type>> = Ptr::null();
pub static mut FLOAT: Ptr<Object<Type>> = Ptr::null();
pub static mut INTEGER: Ptr<Object<Type>> = Ptr::null();
pub static mut SIGNED: Ptr<Object<Type>> = Ptr::null();
pub static mut UNSIGNED: Ptr<Object<Type>> = Ptr::null();

pub static mut BOOLEAN: Ptr<Object<Type>> = Ptr::null();
pub static mut CHAR: Ptr<Object<Type>> = Ptr::null();

pub static mut INT8: Ptr<Object<Type>> = Ptr::null();
pub static mut INT16: Ptr<Object<Type>> = Ptr::null();
pub static mut INT32: Ptr<Object<Type>> = Ptr::null();
pub static mut INT64: Ptr<Object<Type>> = Ptr::null();

pub static mut UINT8: Ptr<Object<Type>> = Ptr::null();
pub static mut UINT16: Ptr<Object<Type>> = Ptr::null();
pub static mut UINT32: Ptr<Object<Type>> = Ptr::null();
pub static mut UINT64: Ptr<Object<Type>> = Ptr::null();

pub static mut FLOAT32: Ptr<Object<Type>> = Ptr::null();
pub static mut FLOAT64: Ptr<Object<Type>> = Ptr::null();


pub unsafe fn init_builtins() {
    TYP = Object::new_null_typ(
        TypeBuilder::new("Type").is_abstract().build()
    );
    TYP.typ = TYP;
    TYP.value.supr = Some(ANY);

    ANY = Object::new(TYP, TypeBuilder::new("Any").is_abstract().build());
    ANY.value.supr = Some(ANY);

    NIL = Object::new(TYP, TypeBuilder::new("Nil")
        .constructor_raw(constructors::nil)
        .supr(ANY).build());

    READER = Object::new(TYP, TypeBuilder::new("Reader")
        .supr(ANY).build());

    LIST = Object::new(TYP, TypeBuilder::new("List")
        .supr(ANY)
        .constructor_raw(List::constructor)
        .size(mem::size_of::<List>())
        .build());

    SYMBOL = Object::new(TYP, TypeBuilder::new("Symbol")
        .supr(ANY)
        .size(mem::size_of::<String>())
        .build());

    NUMBER = Object::new(TYP, TypeBuilder::new("Number")
        .supr(ANY).is_abstract().build());

    REAL = Object::new(TYP, TypeBuilder::new("Real")
        .supr(NUMBER).is_abstract().build());

    FLOAT = Object::new(TYP, TypeBuilder::new("Float")
        .supr(REAL).is_abstract().build());

    INTEGER = Object::new(TYP, TypeBuilder::new("Integer")
        .supr(REAL).is_abstract().build());
    SIGNED = Object::new(TYP, TypeBuilder::new("Signed")
        .supr(INTEGER).is_abstract().build());
    UNSIGNED = Object::new(TYP, TypeBuilder::new("Unsigned")
        .supr(INTEGER).is_abstract().build());

    BOOLEAN = Object::new(TYP, TypeBuilder::new("Boolean")
        .supr(INTEGER).size(mem::size_of::<bool>()).is_bits().build());
    CHAR = Object::new(TYP, TypeBuilder::new("Char")
        .supr(ANY).size(mem::size_of::<char>()).is_bits().build());

    INT8 = Object::new(TYP, TypeBuilder::new("Int8")
        .supr(SIGNED).size(mem::size_of::<i8>()).is_bits().build());
    INT16 = Object::new(TYP, TypeBuilder::new("Int16")
        .supr(SIGNED).size(mem::size_of::<i16>()).is_bits().build());
    INT32 = Object::new(TYP, TypeBuilder::new("Int32")
        .supr(SIGNED).size(mem::size_of::<i32>()).is_bits().build());
    INT64 = Object::new(TYP, TypeBuilder::new("Int64")
        .supr(SIGNED).size(mem::size_of::<i64>()).is_bits().build());

    UINT8 = Object::new(TYP, TypeBuilder::new("UInt8")
        .supr(UNSIGNED).size(mem::size_of::<u8>()).is_bits().build());
    UINT16 = Object::new(TYP, TypeBuilder::new("UInt16")
        .supr(UNSIGNED).size(mem::size_of::<u16>()).is_bits().build());
    UINT32 = Object::new(TYP, TypeBuilder::new("UInt32")
        .supr(UNSIGNED).size(mem::size_of::<u32>()).is_bits().build());
    UINT64 = Object::new(TYP, TypeBuilder::new("UInt64")
        .supr(UNSIGNED).size(mem::size_of::<u64>()).is_bits().build());

    FLOAT32 = Object::new(TYP, TypeBuilder::new("Float32")
        .supr(UNSIGNED).size(mem::size_of::<f32>()).is_bits().build());
    FLOAT64 = Object::new(TYP, TypeBuilder::new("Float64")
        .supr(UNSIGNED).size(mem::size_of::<f64>()).is_bits().build());
}
