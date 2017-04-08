extern crate lisp;


use lisp::lang::*;
use lisp::Context;


#[test]
fn test_runtime() {
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

                    assert!(value.downcast::<Object<Symbol>>().is_some());

                } else if value.typ() == context.KeywordType {

                    assert!(value.downcast::<Object<Keyword>>().is_some());

                } else if value.typ() == context.Float64Type {

                    assert!(value.downcast::<Object<f64>>().is_some());

                }  else if value.typ() == context.Int64Type {

                    assert!(value.downcast::<Object<isize>>().is_some());

                } else if value.typ() == context.UInt64Type {

                    assert!(value.downcast::<Object<usize>>().is_some());

                }
            }
        }
    }
}
