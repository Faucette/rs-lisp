extern crate lisp;


use lisp::lang::*;
use lisp::utils::*;


fn main() {
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
                    println!("{:?}", v.downcast::<Object<Symbol>>().unwrap());
                } else if v.typ() == unsafe {KEYWORD} {
                    println!("{:?}", v.downcast::<Object<Keyword>>().unwrap());
                }
            }
        }
    }
}
