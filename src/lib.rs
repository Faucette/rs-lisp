#![feature(alloc)]
#![feature(const_fn)]
#![feature(collections)]
#![feature(heap_api)]
#![feature(get_type_id)]
#![no_std]


extern crate alloc;
extern crate collections;

extern crate collection_traits;
extern crate hash_map;
extern crate linked_list;
extern crate vector;
extern crate lexer;


pub mod gc;
pub mod lang;
pub mod context;
pub mod ptr;


pub use self::gc::*;
pub use self::lang::*;
pub use self::context::Context;
pub use self::ptr::Ptr;
