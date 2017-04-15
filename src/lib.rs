#![feature(alloc)]
#![feature(const_fn)]
#![feature(collections)]
#![feature(heap_api)]
#![feature(get_type_id)]
//#![no_std]
extern crate core;


extern crate alloc;
#[macro_use]
extern crate collections;

extern crate collection_traits;
extern crate hash_map;
extern crate linked_list;
extern crate vector;


pub mod gc;
pub mod lang;
pub mod context;
mod eval;
mod lhash;
mod ptr;


pub use self::gc::*;
pub use self::lang::*;
pub use self::context::Context;
pub use self::eval::eval;
pub use self::lhash::LHash;
pub use self::ptr::Ptr;
