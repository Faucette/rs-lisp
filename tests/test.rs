extern crate lisp;


use lisp::lang::*;


#[test]
fn test_runtime() {
    let context = Context::new();
    assert!(context.scope.contains("Any"));
}
