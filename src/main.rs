extern crate lisp;


use lisp::lang::*;
use lisp::{eval, Context};


fn main() {
    let context = Context::new();
    let input = "(+ :keyword, symbol, 1, -1, 1.0)".chars().collect();
    let mut reader = context.gc.new_object(context.ReaderType, Reader::new(&context, input));
    let mut values = reader.collect(&context);

    let scope = context.gc.new_object(context.ScopeType, Scope::new(None, None));
    let output = eval(&context, scope, values.first(&context));

    println!("Eval {:?}", output.typ());

    while !(values.is_empty(&context).value()) {
        let value = values.first(&context);
        values = values.pop(&context);

        if value.typ() == context.ListType {
            let mut list = value.downcast::<Object<List>>().unwrap();

            while !(list.is_empty(&context).value()) {
                let value = list.peek(&context);
                list = list.pop(&context);

                if value.typ() == context.SymbolType {

                    println!("{:?} {:?}", value.downcast::<Object<Symbol>>().unwrap(), value.typ());

                } else if value.typ() == context.KeywordType {

                    println!("{:?} {:?}", value.downcast::<Object<Keyword>>().unwrap(), value.typ());

                } else if value.typ() == context.Float64Type {

                    println!("{:?} {:?}", value.downcast::<Object<f64>>().unwrap(), value.typ());

                }  else if value.typ() == context.Int64Type {

                    println!("{:?} {:?}", value.downcast::<Object<isize>>().unwrap(), value.typ());

                } else if value.typ() == context.UInt64Type {

                    println!("{:?} {:?}", value.downcast::<Object<usize>>().unwrap(), value.typ());

                }
            }
        }
    }

    println!("Totel: {:?} bytes", context.gc.total());
}
