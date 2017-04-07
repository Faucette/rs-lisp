extern crate lisp;


use lisp::lang::*;
use lisp::reader::*;


#[test]
fn test_runtime() {
    let reader = Reader::from("(+ a, b)");
    let tokens: Vec<Token> = reader.collect();
    println!("{:?}", tokens);
    assert!(false);
}
