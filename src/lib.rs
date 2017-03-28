#![feature(alloc)]
#![feature(collections)]
#![feature(heap_api)]
#![feature(core_intrinsics)]
#![feature(shared)]
#![feature(get_type_id)]
//#![no_std]
extern crate core;


extern crate alloc;
extern crate collections;

#[macro_use]
extern crate impl_any;
extern crate collection_traits;
extern crate hash_map;
extern crate linked_list;
extern crate vector;


pub mod gc;
pub mod lang;
pub mod utils;
