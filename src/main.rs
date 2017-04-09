extern crate lisp;


use lisp::lang::*;
use lisp::{eval, Context};


fn main() {
    let context = Context::new();
    let input = "(def add (Function (a, b) (number_add a, b))) (add 1, 1)".chars().collect();
    let mut reader = context.gc.new_object(context.ReaderType, Reader::new(&context, input));
    let mut values = reader.collect(&context, context.scope);
    let mut result = context.nil_value.as_value();

    while !values.is_empty(&context).value() {
        result = eval(&context, context.scope, values.first(&context));

        if result.typ() == context.BooleanType {

            println!("{:?}", result.downcast::<Object<bool>>().unwrap());

        } if result.typ() == context.NilType {

            println!("{:?}", result.downcast::<Object<Nil>>().unwrap());

        } if result.typ() == context.SymbolType {

            println!("{:?}", result.downcast::<Object<Symbol>>().unwrap());

        } else if result.typ() == context.KeywordType {

            println!("{:?}", result.downcast::<Object<Keyword>>().unwrap());

        } else if result.typ() == context.Float64Type {

            println!("{:?}", result.downcast::<Object<f64>>().unwrap());

        }  else if result.typ() == context.Int64Type {

            println!("{:?}", result.downcast::<Object<i64>>().unwrap());

        } else if result.typ() == context.UInt64Type {

            println!("{:?}", result.downcast::<Object<u64>>().unwrap());

        } else {

            println!("{:?}", result.typ());

        }

        values = values.pop(&context);
    }

    println!("Totel: {:?} bytes", context.gc.total());
}
