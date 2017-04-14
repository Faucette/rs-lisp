#![feature(alloc)]
#![feature(const_fn)]
#![feature(collections)]
#![feature(heap_api)]
#![feature(get_type_id)]
//#![no_std]
extern crate core;


extern crate alloc;
extern crate collections;

extern crate collection_traits;
extern crate hash_map;
extern crate linked_list;
extern crate vector;
#[macro_use]
extern crate concat_string;


pub mod gc;
pub mod lang;
pub mod context;
mod eval_recur;
mod eval;
mod ptr;


pub use self::gc::*;
pub use self::lang::*;
pub use self::context::Context;
pub use self::eval_recur::eval_recur;
pub use self::eval::eval;
pub use self::ptr::Ptr;
