extern crate lisp;


use lisp::lang::*;
use lisp::utils::*;


#[test]
fn test_runtime() {
    unsafe {
        init_builtins();
    }

    let reader = Reader::from("(+ a, b)");
    //let ast: Vec<Ptr<Value>> = reader.collect();
    //println!("{:?}", ast);
    assert!(false);
}
