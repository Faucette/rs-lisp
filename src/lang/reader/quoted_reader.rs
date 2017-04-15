use collections::string::String;

use ::{Context, LHash, Ptr};
use ::lang::{Value, Object, Scope, List};
use super::reader::Reader;


pub fn quoted_reader(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.first(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch == '"' || ch == '\'' {
        let quote = ch;

        reader.read();

        let mut string = String::new();

        while !reader.done() {
            let ch = reader.peek(0);

            if ch == quote {
                reader.read();
                break;
            } else {
                reader.read();
                string.push(ch);
            }
        }

        let value = if quote == '"' {
            context.gc.new_object(context.StringType, string).as_value()
        } else {
            context.gc.new_object(context.CharType, string.pop().unwrap()).as_value()
        };

        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.true_value.as_value());
        ret_list.push_back_mut(context, value);
        ret_list.as_value()
    } else {
        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.false_value.as_value());
        ret_list.as_value()
    }
}
