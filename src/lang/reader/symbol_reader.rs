use collections::string::String;

use ::Ptr;
use ::Context;
use ::lang::{Value, Object, Keyword, Symbol, List, Nil};
use super::reader::Reader;


pub fn symbol_reader(context: &Context, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.peek(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if !ch.is_whitespace() && ch != ',' {
        let first = ch;

        reader.read();

        let mut string = String::new();

        if first != ':' {
            string.push(ch);
        }

        while !reader.done() {
            let ch = reader.peek(0);

            if ch.is_whitespace() || ch == ',' || ch == ')' {
                break;
            } else {
                reader.read();
                string.push(ch);
            }
        }

        let value =
            if first == ':' {
                context.gc.new_object(context.KeywordType, Keyword::new(string)).as_value()
            } else {
                match string.as_str() {
                    "true" => context.gc.new_object(context.BooleanType, true).as_value(),
                    "false" => context.gc.new_object(context.BooleanType, false).as_value(),
                    "nil" => context.gc.new_object(context.NilType, Nil::new()).as_value(),
                    _ => context.gc.new_object(context.SymbolType, Symbol::new(string)).as_value()
                }
            };

        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.gc.new_object(context.BooleanType, true).as_value());
        ret_list.push_back_mut(context, value);
        ret_list.as_value()
    } else {
        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.gc.new_object(context.BooleanType, false).as_value());
        ret_list.as_value()
    }
}
