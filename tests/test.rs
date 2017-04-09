extern crate lisp;


use lisp::lang::*;
use lisp::{eval, Context};


#[test]
fn test_runtime() {
    let context = Context::new();
    let input = "(def add (Function (a, b) (number_add a, b))) (add 1, 1)".chars().collect();
    let mut reader = context.gc.new_object(context.ReaderType, Reader::new(&context, input));
    let mut values = reader.collect(&context, context.scope);
    let mut result = context.nil_value.as_value();

    while !values.is_empty(&context).value() {
        result = eval(&context, context.scope, values.first(&context));
        values = values.pop(&context);
    }

    assert_eq!(result.downcast::<Object<u64>>().unwrap().value(), &2);
}
