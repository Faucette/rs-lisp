extern crate lisp;


use lisp::lang::*;
use lisp::utils::*;


#[test]
fn test_runtime() {
    unsafe {
        init_typs();
    }
    let input = "(+ a, b, :keyword, symbol)";
    let mut reader = Reader::new(input.chars().collect());
    let ast: Vec<Ptr<Value>> = reader.collect();

    for value in ast {
        if value.typ() == unsafe {LIST} {
            let mut list = value.downcast::<Object<List>>().unwrap();

            while !(**list.is_empty()) {
                let v = list.peek();
                list = list.pop();

                if v.typ() == unsafe {SYMBOL} {
                    assert!(v.downcast::<Object<Symbol>>().is_some());
                } else if v.typ() == unsafe {KEYWORD} {
                    assert!(v.downcast::<Object<Keyword>>().is_some());
                }
            }
        }
    }
}
