extern crate lisp;


use lisp::lang::*;


fn main() {
    let context = Context::new();
    assert!(context.scope.contains("Any"));
}
