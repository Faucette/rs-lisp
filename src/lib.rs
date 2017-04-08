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
extern crate lexer;


pub mod lang;
pub mod utils;


pub use self::lang::*;
pub use self::utils::*;
