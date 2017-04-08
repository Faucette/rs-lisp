extern crate lisp;


use lisp::lang::*;
use lisp::Context;


fn main() {
    let context = Context::new();
    let input = "(+ :keyword, symbol, 1, -1, 1.0)".chars().collect();
    let mut reader = context.gc.new_object(context.ReaderType, Reader::new(input));
    let mut values = reader.collect(&context);

    while !(values.is_empty(&context).value()) {
        let value = values.first(&context);
        values = values.pop(&context);

        if value.typ() == context.ListType {
            let mut list = value.downcast::<Object<List>>().unwrap();

            while !(list.is_empty(&context).value()) {
                let value = list.peek(&context);
                list = list.pop(&context);

                if value.typ() == context.SymbolType {

                    println!("{:?}", value.downcast::<Object<Symbol>>().unwrap());

                } else if value.typ() == context.KeywordType {

                    println!("{:?}", value.downcast::<Object<Keyword>>().unwrap());

                } else if value.typ() == context.Float64Type {

                    println!("{:?}", value.downcast::<Object<f64>>().unwrap());

                }  else if value.typ() == context.Int64Type {

                    println!("{:?}", value.downcast::<Object<isize>>().unwrap());

                } else if value.typ() == context.UInt64Type {

                    println!("{:?}", value.downcast::<Object<usize>>().unwrap());

                }
            }
        }
    }

    println!("Totel: {:?} bytes", context.gc.total());
}
